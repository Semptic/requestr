all: fmt check test clippy

clippy: fmt check
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

check: fmt
	cargo check --all

test: check
	cargo test --all


publish: test
	cd core; cargo publish && \
	cd ../cli; cargo publish 
