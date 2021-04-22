
check:
	cargo c
	cargo fmt -- --check
	cargo clean -p fb-api
	cargo clippy
	cargo t

clippy-nightly:
	cargo +nightly clean -p fb-api
	cargo +nightly clippy