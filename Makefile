all: fmt check test

fmt:
	cargo fmt --all

test:
	cargo test --all

check:
	cargo check --all
