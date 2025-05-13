cargo build
cargo build --target wasm32-unknown-unknown --release
substreams run -e mainnet.eth.streamingfast.io:443 substreams.yaml map_events  --start-block 22450940  --stop-block +1
