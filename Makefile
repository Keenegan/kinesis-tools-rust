.PHONY: build check fmt

build:
	cargo build

check:
	cargo check

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-features -- -D warnings

test:
	cargo test -- --include-ignored

ci: check fmt clippy test

