prepare:
	rustup default nightly
	rustup target add wasm32-unknown-unknown

build-contract:
	cd storage_contract && cargo build --release --target wasm32-unknown-unknown
	wasm-strip storage_contract/target/wasm32-unknown-unknown/release/storage_contract.wasm

test-only:
	cd tests && cargo test --workspace

copy-wasm-file-to-test:
	mkdir -p tests/wasm
	cp storage_contract/target/wasm32-unknown-unknown/release/*.wasm tests/wasm

test: build-contract copy-wasm-file-to-test test-only

clippy:
	cd storage_contract && cargo clippy --all-targets --all
	cd tests && cargo clippy --all-targets --all

check-lint: clippy
	cd storage_contract && cargo fmt --all -- --check
	cd tests && cargo fmt --all -- --check

format:
	cd storage_contract && cargo fmt --all
	cd tests && cargo fmt --all

lint: clippy format
	
clean:
	cd storage_contract && cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm/*.wasm
