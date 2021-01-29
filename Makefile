all: fmt check test

fmt:
	cargo fmt --all

test:
	cargo test --all

check:
	cargo check --all

publish: test
	cd core; cargo publish && \
	cd ../cli; cargo publish 
