#!/bin/sh

NUM=15
CLOSE_TIME=1688750655

#secretcli tx compute instantiate ${NUM} '{"candidates":[{"id":0,"name":"Donald Trump"},{"id":1,"name":"Joe Biden"}],"voters":["secret1q2stawfcwxtfncmk869zf8lczz372zsdrkml32","secret1n32em04ds80k0kt9q3fw5xgj7pv9jdgyj69j7u","secret1q2stawfcwxtfncmk869zf8lczz372zsdrkml31"],"close_time":1682800823}' --from myWallet --label test${NUM} -y

ADDR=secret1rxuqefhcu8270mcjvpg87yl4uwnf5qkt3q5uah
secretcli query compute query ${ADDR} '{"get_name": {}}'
secretcli query compute query ${ADDR} '{"get_candidate_list": {}}'
secretcli query compute query ${ADDR} '{"get_voters_count": {}}'
secretcli query compute query ${ADDR} '{"get_close_time": {}}'
secretcli query compute query ${ADDR} '{"get_results": {}}'

# secretcli tx compute execute ${ADDR} '{"submit_vote": {"candidate_id": 0}}' --from myWallet
