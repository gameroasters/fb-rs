
check:
	cargo c
	cargo fmt -- --check
	cargo clippy
	cargo t

check-nightly:
	cargo +nightly clippy