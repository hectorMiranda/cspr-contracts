
## Make commands
### prepare
Adds wasm to the cargo compilation targets.

### build-contract
Builds the contracts using the nightly toolchain with wasm as the compilation target.

### test-only
Run all tests inside the workspace.

### copy-wasm-file-to-test
Copies the `.wasm` files into `/tests/wasm` folder, where the test framework is set to look for them.

### test
Executes the `build-contract` and `copy-wasm-file-to-test`, then `test-only` commands.

### clippy
Executes the clippy linter on the contract and test codes.

### lint
Runs `clippy` and `format`

### check-lint
Runs `clippy` then executes `fmt` with `--check` enabled. Gives errors for clippy warnings.

### format
Applies formatting to the codes.

### clean
Artifact removal command. (`.wasm` files, `target` folders)

## Rust version
This contract was compiled and ran during development using rustc nightly-2023-03-25

## Casper contract sdk version
casper-types = "2.0.0"
casper-contract = "2.0.0"
casper-engine-test-support = "4.0.0"


## Casper client related commands

The following commands were tested againts the testnet network

### Get state root hash from a testnet node
casper-client get-state-root-hash --node-address http://88.99.100.42:7777


### Get global state

Get global state
 casper-client query-global-state --node-address http://88.99.100.42:7777 --state-root-hash 14d07b30d79db7c40b67e4ae4f5914694ad29a14dac4309f6f240438c573f798 --key account-hash-c7075a2366f564b6aa4d2a4907a81dd4002e896e0bee0f9cc9726d713bb160cc


### Deploy contract

casper-client put-deploy --node-address http://88.99.100.42:7777 --chain-name casper-test --secret-key ./secret_key.pem --payment-amount 50000000000  --session-path ./storage_contract.wasm --session-entry-point "store_data" --session-arg "name:string='storage'"

### Retrieve specific contract details

1. get the new state root hash

casper-client get-state-root-hash --node-address http://88.99.100.42:7777


2. get network state and storage contract hashs

casper-client query-global-state --node-address http://88.99.100.42:7777 --state-root-hash 9811d09e671475aaec1dc53cc6a53877d48394a61871a65366c5268735699dff --key account-hash-c7075a2366f564b6aa4d2a4907a81dd4002e896e0bee0f9cc9726d713bb160cc

