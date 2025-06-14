# https://docs.sui.io/references/cli/client
alias b := build
CID := "QmY7vH8P76VwbvXtV2hqXn3Juv6m3Fz7A3KFehrZCh4V4g"

PACKAGE := shell("tq --file Move.lock .env.local.latest-published-id")
ADDRESS := shell('sui client addresses --json  | jq  ".activeAddress" ')

build:
	sui move build

dry_mint:
	sui client call --package {{ PACKAGE }} --module ipcm --function mint --dry-run --args {{ CID }} "['Public GPG Key']"   

mint: # let the user overwrite the args
	sui client call --package {{ PACKAGE }} --module ipcm --function mint --args {{ CID }} "['Public GPG Key']"   

publish:
	sui client publish --gas-budget 5000000000 --json

drip:
	sui client faucet

objects: 
	sui client objects {{ ADDRESS }} #--json

ipcms: # get just ipcms
	sui client objects {{ ADDRESS }} --json | jq '.[].data | select(.type|test("IPCM")) | {id:.objectId,cid:.content.fields.cid.fields}'

localnet:
	RUST_LOG="off,sui_node=error" sui start --with-faucet --force-regenesis

addy:
	echo {{ ADDRESS }}

package:
	echo {{ PACKAGE }}

# read:
# 	sui client call --package {{ PACKAGE }} --module ipcm --function read_ipcm --args 0x22b4b8282ce2706af37e5e2f939188149ac310cfc1da1f87a55530cacb1af546 #--json
	
# update:
# 	sui client call --package {{ PACKAGE }} --module ipcm --function update --args 0x22b4b8282ce2706af37e5e2f939188149ac310cfc1da1f87a55530cacb1af546 "GOTTEM" '[]' 'false' #--json


# curl -X POST http://127.0.0.1:9000 \
# -H "Content-Type: application/json" \
# -d '{
#   "jsonrpc": "2.0",
#   "id": 1,
#   "method": "suix_queryEvents",
#   "params": [
#     {
#       "MoveModule": {
#         "package": "0x4d859c0a57f08f182eb559ecc6c1df3756b07a67477cba87649cd02acc43e1b7",
#         "module": "ipcm",
#         "type": "0x4d859c0a57f08f182eb559ecc6c1df3756b07a67477cba87649cd02acc43e1b7::ipcm::UpdatedIPCM"
#       }
#     },
#     null,
#     3,
#     false
#   ]
# }' | jq ".result.data"
