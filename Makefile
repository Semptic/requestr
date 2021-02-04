all: fmt test clippy

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

check-fmt:
	cargo fmt --all -- --check

check: 
	cargo check --all

test: check
	cargo test --all

publish: check-fmt test clippy
	cd core; cargo publish && \
	cd ../cli; cargo publish 
