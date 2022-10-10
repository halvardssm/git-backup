run:
	cargo run -- --config=./git_sync_config.yaml
build:
	cargo build --release --target x86_64-unknown-linux-gnu
fmt:
	cargo fmt
