#!/usr/bin/env bash
#
# Patches the SPL crates for developing against a local paychains monorepo
#

paychains_dir=$1
if [[ -z $paychains_dir ]]; then
  echo "Usage: $0 <path-to-paychains-monorepo>"
  exit 1
fi

workspace_crates=(
  Cargo.toml
)

if [[ ! -r "$paychains_dir"/scripts/read-cargo-variable.sh ]]; then
  echo "$paychains_dir is not a path to the paychains monorepo"
  exit 1
fi

set -e

paychains_dir=$(cd "$paychains_dir" && pwd)
cd "$(dirname "$0")"

source "$paychains_dir"/scripts/read-cargo-variable.sh
paychains_ver=$(readCargoVariable version "$paychains_dir"/sdk/Cargo.toml)

echo "Patching in $paychains_ver from $paychains_dir"
echo
for crate in "${workspace_crates[@]}"; do
  if grep -q '\[patch.crates-io\]' "$crate"; then
    echo "$crate is already patched"
  else
    cat >> "$crate" <<PATCH
[patch.crates-io]
paychains-account-decoder = {path = "$paychains_dir/account-decoder" }
paychains-banks-client = { path = "$paychains_dir/banks-client"}
paychains-banks-server = { path = "$paychains_dir/banks-server"}
paychains-bpf-loader-program = { path = "$paychains_dir/programs/bpf_loader" }
paychains-clap-utils = {path = "$paychains_dir/clap-utils" }
paychains-cli-config = {path = "$paychains_dir/cli-config" }
paychains-cli-output = {path = "$paychains_dir/cli-output" }
paychains-client = { path = "$paychains_dir/client"}
paychains-core = { path = "$paychains_dir/core"}
paychains-logger = {path = "$paychains_dir/logger" }
paychains-notifier = { path = "$paychains_dir/notifier" }
paychains-remote-wallet = {path = "$paychains_dir/remote-wallet" }
paychains-program = { path = "$paychains_dir/sdk/program" }
paychains-program-test = { path = "$paychains_dir/program-test" }
paychains-runtime = { path = "$paychains_dir/runtime" }
paychains-sdk = { path = "$paychains_dir/sdk" }
paychains-stake-program = { path = "$paychains_dir/programs/stake" }
paychains-transaction-status = { path = "$paychains_dir/transaction-status" }
paychains-version = { path = "$paychains_dir/version" }
paychains-vote-program = { path = "$paychains_dir/programs/vote" }
PATCH
  fi
done

./update-paychains-dependencies.sh "$paychains_ver"
