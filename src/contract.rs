use cosmwasm_std::{entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, StdResult};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Candidate, CANDIDATE_RESULT, CandidateResult, config, config_read, State, VOTERS};

use cosmwasm_std::Timestamp;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let state = State {
        name: _msg.name,
        candidates_list: _msg.candidates,
        voters_addresses: _msg.voters,
        close_time: _msg.close_time,
    };

    // Create candidate result
    for candidate in &state.candidates_list {
        let id = candidate.id;
        let value: u16 = 0;
        CANDIDATE_RESULT.insert(deps.storage, &id, &value)?;
    }

    // Create voters addresses default state
    for voter in &state.voters_addresses {
        let addr = voter;
        VOTERS.insert(deps.storage, &addr, &false)?;
    }

    deps.api.debug(format!("Contract was initialized by {}", _info.sender).as_str());

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, StdError> {
    match msg {
        ExecuteMsg::SubmitVote { candidate_id } => try_submit_vote(deps, _env, _info, candidate_id),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::GetName {} => to_binary(&query_get_name(deps)?),
        QueryMsg::GetCandidateList {} => to_binary(&query_candidate_list(deps)?),
        QueryMsg::GetVotersCount {} => to_binary(&query_voters_count(deps)?),
        QueryMsg::GetCloseTime {} => to_binary(&query_close_time(deps)?),
        QueryMsg::GetResults {} => to_binary(&query_results(deps, _env)?),
    }
}

pub fn try_submit_vote(deps: DepsMut,
                       _env: Env,
                       _info: MessageInfo,
                       candidate_id: u16,
) -> Result<Response, StdError> {
    let state = config_read(deps.storage).load()?;
    let current_time: Timestamp = _env.block.time;

    // Check for current time
    if current_time.seconds() >= state.close_time {
        return Err(StdError::generic_err("Voting close time has passed, voting is closed"));
    }

    // Check if voter exists in voters list
    let voter_exists = VOTERS.contains(deps.storage, &_info.sender);
    if !voter_exists {
        return Err(StdError::generic_err("Voter is not eligible to vote in this smart contract"));
    }

    // Check if voter already voted
    let voter_voted = VOTERS.get(deps.storage, &_info.sender).expect("Voter not found");
    if voter_voted {
        return Err(StdError::generic_err("Voter has already voted"));
    }

    let current_vote_count = CANDIDATE_RESULT.get(deps.storage, &candidate_id).expect("Invalid candidate id").clone();

    VOTERS.insert(deps.storage, &_info.sender, &true)?;
    CANDIDATE_RESULT.insert(deps.storage, &candidate_id, &(current_vote_count + 1))?;

    Ok(Response::new())
}

fn query_get_name(deps: Deps) -> StdResult<String> {
    let state = config_read(deps.storage).load()?;
    Ok(state.name)
}

fn query_candidate_list(deps: Deps) -> Result<Vec<Candidate>, StdError> {
    let state = config_read(deps.storage).load()?;
    Ok(state.candidates_list)
}

fn query_voters_count(deps: Deps) -> Result<u32, StdError> {
    let voters_iterator = VOTERS.iter_keys(deps.storage)?;
    let count: u32 = voters_iterator.count() as u32;
    Ok(count)
}

fn query_close_time(deps: Deps) -> StdResult<u64> {
    let state = config_read(deps.storage).load()?;
    Ok(state.close_time)
}

fn query_results(deps: Deps, _env: Env) -> Result<Vec<CandidateResult>, StdError> {
    let state = config_read(deps.storage).load()?;
    let current_time: Timestamp = _env.block.time;

    if current_time.seconds() < state.close_time {
        return Err(StdError::generic_err("Voting has not been closed yet"));
    }

    let mut candidate_results = Vec::new();

    let candidate_iter = CANDIDATE_RESULT.iter(deps.storage)?;
    for elem in candidate_iter {
        let content = elem.unwrap();
        candidate_results.push(CandidateResult::new(content.0, content.1));
    }

    Ok(candidate_results)
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies};
    use cosmwasm_std::{Addr, coins};

    #[test]
    fn init_test() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let alice: Candidate = Candidate {
            id: 0,
            name: "Alice".to_string(),
        };

        let bob: Candidate = Candidate {
            id: 1,
            name: "Bob".to_string(),
        };

        let voter1: Addr = Addr::unchecked("addr1".to_string());
        let voter2: Addr = Addr::unchecked("addr2".to_string());
        let voter3: Addr = Addr::unchecked("addr3".to_string());

        let msg = InstantiateMsg {
            name: "Test voting 001".to_string(),
            candidates: vec![alice, bob],
            voters: vec![voter1, voter2, voter3],
            close_time: 2682000000,
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // Check for response
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Query responses
        let voting_name = query_get_name(deps.as_ref()).unwrap();
        assert_eq!("Test voting 001".to_string(), voting_name);

        let close_time = query_close_time(deps.as_ref()).unwrap();
        assert_eq!(2682000000, close_time);

        let candidates = query_candidate_list(deps.as_ref()).unwrap();
        assert_eq!(0, candidates[0].id);
        assert_eq!("Bob", candidates[1].name);

        // We expect error because voting did not finished yet
        let voting_result = query_results(deps.as_ref(), env).unwrap_err();
        assert_eq!("Generic error: Voting has not been closed yet".to_string(), voting_result.to_string());
    }

    #[test]
    fn vote_test() {
        let mut deps = mock_dependencies();
        let env = mock_env();

        let alice: Candidate = Candidate {
            id: 0,
            name: "Alice".to_string(),
        };

        let bob: Candidate = Candidate {
            id: 1,
            name: "Bob".to_string(),
        };

        let voter1: Addr = Addr::unchecked("voter1".to_string());
        let voter2: Addr = Addr::unchecked("voter2".to_string());
        let voter3: Addr = Addr::unchecked("voter3".to_string());

        let msg = InstantiateMsg {
            name: "Test voting 002".to_string(),
            candidates: vec![alice, bob],
            voters: vec![voter1, voter2, voter3],
            close_time: 2682000000,
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // Check for response
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let voter_info = mock_info("voter2", &coins(1000, "earth"));

        // Try to unwrap for the 1st time - successfully vote
        let vote1_msg = ExecuteMsg::SubmitVote { candidate_id: 0 };
        let submit_res1 = execute(deps.as_mut(), env.clone(), voter_info.clone(), vote1_msg).unwrap();
        assert_eq!(0, submit_res1.messages.len());

        // Try to unwrap error for the 2nd time - cannot vote twice
        let vote2_msg = ExecuteMsg::SubmitVote { candidate_id: 0 };
        let submit_res2 = execute(deps.as_mut(), env.clone(), voter_info.clone(), vote2_msg).unwrap_err();
        assert_ne!(0, submit_res2.to_string().len());

        // The same applies for the other candidates
        let vote3_msg = ExecuteMsg::SubmitVote { candidate_id: 1 };
        let submit_res3 = execute(deps.as_mut(), env.clone(), voter_info.clone(), vote3_msg).unwrap_err();
        assert_ne!(0, submit_res3.to_string().len());

        // Random non-eligible address cannot vote
        let voter_info2 = mock_info("non-eligible-voter", &coins(1000, "earth"));
        let vote4_msg = ExecuteMsg::SubmitVote { candidate_id: 1 };
        let submit_res4 = execute(deps.as_mut(), env.clone(), voter_info2, vote4_msg).unwrap_err();
        assert_ne!(0, submit_res4.to_string().len());
    }

    #[test]
    fn close_time_test() {
        let mut deps = mock_dependencies();

        let alice: Candidate = Candidate {
            id: 0,
            name: "Alice".to_string(),
        };

        let bob: Candidate = Candidate {
            id: 1,
            name: "Bob".to_string(),
        };

        let voter1: Addr = Addr::unchecked("voter1".to_string());
        let voter2: Addr = Addr::unchecked("voter2".to_string());
        let voter3: Addr = Addr::unchecked("voter3".to_string());

        let msg = InstantiateMsg {
            name: "Test voting 002".to_string(),
            candidates: vec![alice, bob],
            voters: vec![voter1, voter2, voter3],
            close_time: 4195,   // Close time set too low for the purpose of this test
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // Check for response
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // Because voting has been closed at the moment as it was created, each candidate should have 0 votes
        let candidate1 = CANDIDATE_RESULT.get(deps.as_ref().storage, &0).expect("Invalid candidate");
        assert_eq!(0, candidate1);

        let candidate2 = CANDIDATE_RESULT.get(deps.as_ref().storage, &0).expect("Invalid candidate");
        assert_eq!(0, candidate2);
    }
}
