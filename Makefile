
check:
	cargo c
	cargo fmt -- --check
	cargo clean -p fb
	cargo clippy
	cargo t

clippy-nightly:
	cargo +nightly clean -p fb
	cargo +nightly clippy