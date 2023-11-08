#!/bin/sh
set -e

# This script installs 'gdpack' by downloading prebuilt binaries from the
# project's GitHub releases page. By default the latest version is installed,
# but a different release can be used instead by setting $GDPACK_VERSION.
#
# The script will set up a 'gdpack' cache at '$HOME/.gdpack'. This behavior can
# be customized by setting '$GDPACK_HOME' prior to running the script. Existing
# Godot artifacts cached in a 'gdpack' store won't be lost, but this script will
# overwrite any 'gdpack' binary artifacts in '$GDPACK_HOME/bin'.

# ------------------------------ Define: Cleanup ----------------------------- #

trap cleanup EXIT

cleanup() {
    if [ -d "${GDPACK_TMP=}" ]; then
        rm -rf "${GDPACK_TMP}"
    fi
}

# ------------------------------ Define: Logging ----------------------------- #

info() {
    if [ "$1" != "" ]; then
        echo info: "$@"
    fi
}

warn() {
    if [ "$1" != "" ]; then
        echo warning: "$1"
    fi
}

error() {
    if [ "$1" != "" ]; then
        echo error: "$1" >&2
    fi
}

fatal() {
    error "$1"
    exit 1
}

unsupported_platform() {
    error "$1"
    echo "See https://github.com/coffeebeats/gdpack/blob/main/docs/installation.md#install-from-source for instructions on compiling from source."
    exit 1
}

# ------------------------------- Define: Usage ------------------------------ #

usage() {
    cat <<EOF
gdpack-install: Install 'gdpack' for managing Godot addons.

Usage: gdpack-install [OPTIONS]

NOTE: The following dependencies are required:
    - curl OR wget
    - grep
    - sha256sum OR shasum
    - tar/unzip
    - tr
    - uname

Available options:
    -h, --help          Print this help and exit
    -v, --verbose       Print script debug info (default=false)
    --no-modify-path    Do not modify the \$PATH environment variable
EOF
    exit
}

check_cmd() {
    command -v "$1" >/dev/null 2>&1
}

need_cmd() {
    if ! check_cmd "$1"; then
        fatal "required command not found: '$1'"
    fi
}

# ------------------------------ Define: Params ------------------------------ #

parse_params() {
    MODIFY_PATH=1

    while :; do
        case "${1:-}" in
        -h | --help) usage ;;
        -v | --verbose) set -x ;;

        --no-modify-path) MODIFY_PATH=0 ;;

        -?*) fatal "Unknown option: $1" ;;
        "") break ;;
        esac
        shift
    done

    return 0
}

parse_params "$@"

# ------------------------------ Define: Version ----------------------------- #

GDPACK_VERSION="${GDPACK_VERSION=0.0.2}" # x-release-please-version
GDPACK_VERSION="v${GDPACK_VERSION#v}"

# ----------------------------- Define: Platform ----------------------------- #

need_cmd tr
need_cmd uname

GDPACK_OS="$(echo "${GDPACK_OS=$(uname -s)}" | tr '[:upper:]' '[:lower:]')"
case "$GDPACK_OS" in
darwin*) GDPACK_OS="macos" ;;
linux*) GDPACK_OS="linux" ;;
mac | macos | osx) GDPACK_OS="macos" ;;
cygwin*) GDPACK_OS="windows" ;;
msys* | mingw64*) GDPACK_OS="windows" ;;
uwin* | win*) GDPACK_OS="windows" ;;
*) unsupported_platform "no prebuilt binaries available for operating system: $GDPACK_OS" ;;
esac

GDPACK_ARCH="$(echo ${GDPACK_ARCH=$(uname -m)} | tr '[:upper:]' '[:lower:]')"
case "$GDPACK_ARCH" in
aarch64 | arm64)
    GDPACK_ARCH="arm64"
    if [ "$GDPACK_OS" != "macos" ]; then
        fatal "no prebuilt '$GDPACK_ARCH' binaries available for operating system: $GDPACK_OS"
    fi

    ;;
amd64 | x86_64) GDPACK_ARCH="x86_64" ;;
*) unsupported_platform "no prebuilt binaries available for CPU architecture: $GDPACK_ARCH" ;;
esac

GDPACK_ARCHIVE_EXT=""
case "$GDPACK_OS" in
windows) GDPACK_ARCHIVE_EXT="zip" ;;
*) GDPACK_ARCHIVE_EXT="tar.gz" ;;
esac

GDPACK_ARCHIVE="gdpack-$GDPACK_VERSION-$GDPACK_OS-$GDPACK_ARCH.$GDPACK_ARCHIVE_EXT"

# ------------------------------- Define: Store ------------------------------ #

GDPACK_HOME_PREV="${GDPACK_HOME_PREV=}" # save for later in script

GDPACK_HOME="${GDPACK_HOME=}"
if [ "$GDPACK_HOME" = "" ]; then
    if [ "${HOME=}" = "" ]; then
        fatal "both '\$GDPACK_HOME' and '\$HOME' unset; one must be specified to determine 'gdpack' installation path"
    fi

    GDPACK_HOME="$HOME/.gdpack"
fi

info "using 'gdpack' store path: '$GDPACK_HOME'"

# ----------------------------- Define: Download ----------------------------- #

need_cmd grep
need_cmd mktemp

GDPACK_TMP=$(mktemp -d --tmpdir gdpack-XXXXXXXXXX)
cd "$GDPACK_TMP"

GDPACK_RELEASE_URL="https://github.com/coffeebeats/gdpack/releases/download/$GDPACK_VERSION"

download_with_curl() {
    curl \
        --fail \
        --location \
        --parallel \
        --retry 3 \
        --retry-delay 1 \
        --show-error \
        --silent \
        -o "$GDPACK_ARCHIVE" \
        "$GDPACK_RELEASE_URL/$GDPACK_ARCHIVE" \
        -o "checksums.txt" \
        "$GDPACK_RELEASE_URL/checksums.txt"
}

download_with_wget() {
    wget -q -t 4 -O "$GDPACK_ARCHIVE" "$GDPACK_RELEASE_URL/$GDPACK_ARCHIVE" 2>&1
    wget -q -t 4 -O "checksums.txt" "$GDPACK_RELEASE_URL/checksums.txt" 2>&1
}

if check_cmd curl; then
    download_with_curl
elif check_cmd wget; then
    download_with_wget
else
    fatal "missing one of 'curl' or 'wget' commands"
fi

# -------------------------- Define: Verify checksum ------------------------- #

verify_with_sha256sum() {
    cat "checksums.txt" | grep "$GDPACK_ARCHIVE" | sha256sum --check --status
}

verify_with_shasum() {
    cat "checksums.txt" | grep "$GDPACK_ARCHIVE" | shasum -a 256 -p --check --status
}

if check_cmd sha256sum; then
    verify_with_sha256sum
elif check_cmd shasum; then
    verify_with_shasum
else
    fatal "missing one of 'sha256sum' or 'shasum' commands"
fi

# ------------------------------ Define: Extract ----------------------------- #

case "$GDPACK_OS" in
windows)
    need_cmd unzip

    mkdir -p "$GDPACK_HOME/bin"
    unzip -u "$GDPACK_ARCHIVE" -d "$GDPACK_HOME/bin"
    ;;
*)
    need_cmd tar

    mkdir -p "$GDPACK_HOME/bin"
    tar -C "$GDPACK_HOME/bin" --no-same-owner -xzf "$GDPACK_ARCHIVE"
    ;;
esac

info "successfully installed 'gdpack@$GDPACK_VERSION' to '$GDPACK_HOME/bin'"

if [ $MODIFY_PATH -eq 0 ]; then
    exit 0
fi

# The $PATH modification and $GDPACK_HOME export is already done.
if check_cmd gdpack && [ "$GDPACK_HOME_PREV" != "" ]; then
    exit 0
fi

# Simplify the exported $GDPACK_HOME if possible.
if [ "$HOME" != "" ]; then
    case "$GDPACK_HOME" in
    $HOME*) GDPACK_HOME="\$HOME${GDPACK_HOME#$HOME}" ;;
    esac
fi

CMD_EXPORT_HOME="export GDPACK_HOME=\"$GDPACK_HOME\""
CMD_MODIFY_PATH="export PATH=\"\$GDPACK_HOME/bin:\$PATH\""

case $(basename $SHELL) in
sh) OUT="$HOME/.profile" ;;
bash) OUT="$HOME/.bashrc" ;;
zsh) OUT="$HOME/.zshenv" ;;
*)
    echo ""
    echo "Add the following to your shell profile script:"
    echo "    $CMD_EXPORT_HOME"
    echo "    $CMD_MODIFY_PATH"
    ;;
esac

if [ "$OUT" != "" ]; then
    if [ -f "$OUT" ] && $(cat "$OUT" | grep -q 'export GDPACK_HOME'); then
        info "Found 'GDPACK_HOME' export in shell Rc file; skipping modification."
        exit 0
    fi

    if [ -f "$OUT" ] && [ "$(tail -n 1 "$OUT")" != "" ]; then
        echo "" >>"$OUT"
    fi

    echo "# Added by 'gdpack' install script." >>"$OUT"
    echo "$CMD_EXPORT_HOME" >>"$OUT"
    echo "$CMD_MODIFY_PATH" >>"$OUT"

    info "Updated shell Rc file: $OUT\n      Open a new terminal to start using 'gdpack'."
fi
