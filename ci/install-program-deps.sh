#!/usr/bin/env bash

set -e

source ci/rust-version.sh stable
source ci/paychains-version.sh install

set -x

cargo --version
cargo install rustfilt || true
cargo install honggfuzz --version=0.5.54 --force || true

cargo +"$rust_stable" build-bpf --version
