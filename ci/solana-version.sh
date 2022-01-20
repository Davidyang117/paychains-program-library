#
# This file maintains the paychains versions for use by CI.
#
# Obtain the environment variables without any automatic updating:
#   $ source ci/paychains-version.sh
#
# Obtain the environment variables and install update:
#   $ source ci/paychains-version.sh install

# Then to access the paychains version:
#   $ echo "$paychains_version"
#

if [[ -n $PAYCHAINS_VERSION ]]; then
  paychains_version="$PAYCHAINS_VERSION"
else
  paychains_version=v1.9.2
fi

export paychains_version="$paychains_version"
export PATH="$HOME"/.local/share/paychains/install/active_release/bin:"$PATH"

if [[ -n $1 ]]; then
  case $1 in
  install)
    sh -c "$(curl -sSfL https://release.paychains.com/$paychains_version/install)"
    paychains --version
    ;;
  *)
    echo "paychains-version.sh: Note: ignoring unknown argument: $1" >&2
    ;;
  esac
fi
