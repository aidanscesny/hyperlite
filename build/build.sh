#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <debug|release>"
    exit 1
fi

BUILD_MODE="$1"

if [[ "${BUILD_MODE}" != "debug" && "${BUILD_MODE}" != "release" ]]; then
    echo "Invalid build mode: ${BUILD_MODE}"
    echo "Expected: debug or release"
    exit 1
fi

echo "========================================"
echo " Building Hyperlite (${BUILD_MODE})"
echo "========================================"

"${ROOT_DIR}/build/scripts/userspace.sh" "${BUILD_MODE}"
"${ROOT_DIR}/build/scripts/kernel.sh"
"${ROOT_DIR}/build/scripts/initramfs.sh" "${BUILD_MODE}"

echo
echo "Build complete."
