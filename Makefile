network 				?= devnet  # network := devnet|mainnet|testnet
sender 					?= juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
build_dir 				?= ./builds
wasm_filename 			?= cw_faucet.wasm

# build optimized WASM artifact
build:
	./bin/build

# deploy WASM file (generated from `make build`)
deploy:
	./bin/deploy ./artifacts/$(wasm_filename) $(network) $(sender) $(tag)

# instantiate last contract to be deployed using code ID in release dir code-id file
instantiate:
	./bin/instantiate $(network) $(sender) $(tag) '{"acl_address":"$(acl)"}'

instantiate-dev:
	./bin/instantiate $(network) $(sender) $(tag) '{"acl_address":"$(acl)","params":[{"token":{"native":{"denom":"ujunox"}},"interval":"300"}]}'

# run all unit tests
test:
	RUST_BACKTRACE=1 cargo unit-test

# Generate the contract's JSONSchema JSON files in schemas/
schemas:
	cargo schema

# Run/start local "devnet" validator docker image	
devnet:
	./bin/devnet

transfer:
	./client.sh transfer $(network) $(tag) $(sender) $(recipient) $(amount)

configure:
	./client.sh configure $(network) $(tag) $(sender) $(interval)

select:
	./client.sh select $(network) $(tag)
