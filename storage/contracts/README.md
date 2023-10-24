
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