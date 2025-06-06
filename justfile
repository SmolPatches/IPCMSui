# https://docs.sui.io/references/cli/client
alias b := build
CID := "QmY7vH8P76VwbvXtV2hqXn3Juv6m3Fz7A3KFehrZCh4V4g"
PACKAGE := "0xe4c734879d563b2a7e7e73cd7c0fa1ea149a4f50b6e5929a24a0e24a7bd903f1"
ADDRESS := shell('sui client addresses --json  | jq  ".activeAddress" ')

build:
	sui move build

dry_mint:
	sui client call --package {{ PACKAGE }} --module ipcm --function mint --dry-run --args {{ CID }} "['Public GPG Key']"   

mint:
	sui client call --package {{ PACKAGE }} --module ipcm --function mint --args {{ CID }} "['Public GPG Key']"   

publish:
	sui client publish --gas-budget 5000000000 > addr.txt 

drip:
	sui client faucet

objects: 
	sui client objects {{ ADDRESS }} --json | jq ".[].data.type"
full_objects:
	sui client objects {{ ADDRESS }} --json | jq ".[].data"
