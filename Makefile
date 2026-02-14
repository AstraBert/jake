.PHONY: build test clippy clippy-fix format format-check version-bump npm-publish

build:
	cargo build --target-dir target/

test:
	$(info ****************** running tests ******************)
	rm -rf .rag-rs-cache/ && cargo test

clippy:
	$(info ****************** running clippy in check mode ******************)
	cargo clippy

clippy-fix:
	$(info ****************** running clippy in fix mode ******************)
	cargo clippy --fix --bin "rag-rs"

format:
	$(info ****************** running rustfmt in fix mode ******************)
	cargo fmt

format-check:
	$(info ****************** running rustfmt in check mode ******************)
	cargo fmt --check

version-bump:
	$(info ****************** bumping version in package.json and Cargo.toml ******************)
	python3 scripts/version_bump.py

npm-publish:
	$(info ****************** login and publish to npm ******************)
	$(info ****************** meant for manual usage ******************)
	bash scripts/login_and_publish_to_npm.sh
