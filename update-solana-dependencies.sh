#!/usr/bin/env bash
#
# Updates the paychains version in all the SPL crates
#

paychains_ver=$1
if [[ -z $paychains_ver ]]; then
  echo "Usage: $0 <new-paychains-version>"
  exit 1
fi

cd "$(dirname "$0")"

sed -i'' -e "s#paychains_version=v.*#paychains_version=v${paychains_ver}#" ./ci/paychains-version.sh

declare tomls=()
while IFS='' read -r line; do tomls+=("$line"); done < <(find . -name Cargo.toml)

crates=(
  paychains-account-decoder
  paychains-banks-client
  paychains-banks-server
  paychains-bpf-loader-program
  paychains-clap-utils
  paychains-cli-config
  paychains-cli-output
  paychains-client
  paychains-core
  paychains-logger
  paychains-notifier
  paychains-program
  paychains-program-test
  paychains-remote-wallet
  paychains-runtime
  paychains-sdk
  paychains-stake-program
  paychains-transaction-status
  paychains-vote-program
  paychains-version
)

set -x
for crate in "${crates[@]}"; do
  sed -E -i'' -e "s#(${crate} = \")(=?).*#\1\2${paychains_ver}\"#" "${tomls[@]}"
done
