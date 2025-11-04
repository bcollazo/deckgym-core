fix:
	cargo fmt
	cargo clippy --all-targets --features tui --allow-dirty --fix -- -D warnings