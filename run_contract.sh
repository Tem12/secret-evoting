NUM=19
secretcli tx compute instantiate ${NUM} '{"name":"BDA e-voting","candidates":[{"id":0,"name":"Alice"},{"id":1,"name":"Bob"},{"id":2,"name":"Carol"},{"id":3,"name":"Dave"}],"voters":["secret1q2stawfcwxtfncmk869zf8lczz372zsdrkml32","secret1n32em04ds80k0kt9q3fw5xgj7pv9jdgyj69j7u","secret1q2stawfcwxtfncmk869zf8lczz372zsdrkml31","secret1k743hdlyq355z7ufehlpq858q5crexh96av7hx"],"close_time":1688750655}' --from myWallet --label test_voting${NUM} -y
