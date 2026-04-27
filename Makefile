fix:
	cargo fmt
	cargo clippy --all-targets --features "tui,test-utils" --allow-dirty --fix -- -D warnings