#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")"
cargo fmt -- --check
cargo clippy
cargo build
cargo build-bpf

if [[ $1 = -v ]]; then
  export RUST_LOG=paychains=debug
fi

cargo test
cargo test-bpf
