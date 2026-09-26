#!/usr/bin/env bash

set -euo pipefail

BUILD_MODE="$1"

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

echo "Building Hyperlite userspace (${BUILD_MODE})..."

if [[ "${BUILD_MODE}" == "release" ]]; then
    cargo build \
        --manifest-path "${ROOT_DIR}/Cargo.toml" \
        --target x86_64-unknown-linux-musl \
        --release
else
    cargo build \
        --manifest-path "${ROOT_DIR}/Cargo.toml" \
        --target x86_64-unknown-linux-musl
fi

echo "Userspace built."
