#!/usr/bin/env just --justfile

release:
    cargo build --release

lint:
    cargo clippy --all --fix --allow-dirty --allow-staged

test:
    cargo test

fmt:
    cargo fmt --all
    prettier --write ./*.md

login:
    cargo login --registry crates-io

publish:
    cargo publish --registry crates-io
