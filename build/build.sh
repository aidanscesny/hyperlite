#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Defaults
BUILD_MODE="release"
JOBS="$(( $(nproc) / 2 ))"

# Don't somehow produce -j0 on a single-core system.
if (( JOBS < 1 )); then
    JOBS=1
fi

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  --build-mode=<debug|release>  Build mode (default: release)"
    echo "  --jobs=<N>                    Parallel kernel build jobs (default: nproc / 2)"
    echo "  -h, --help                    Show this help message"
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --build-mode=*)
            BUILD_MODE="${1#*=}"
            ;;
        --jobs=*)
            JOBS="${1#*=}"
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo "Unknown argument: $1" >&2
            echo >&2
            usage >&2
            exit 1
            ;;
    esac

    shift
done

# Validate build mode
case "${BUILD_MODE}" in
    debug|release)
        ;;
    *)
        echo "Invalid build mode: ${BUILD_MODE}" >&2
        echo "Expected: debug or release" >&2
        exit 1
        ;;
esac

# Validate job count
if [[ ! "${JOBS}" =~ ^[1-9][0-9]*$ ]]; then
    echo "Invalid job count: ${JOBS}" >&2
    echo "Expected a positive integer" >&2
    exit 1
fi

echo "========================================"
echo " Building Hyperlite"
echo "========================================"
echo " Build mode : ${BUILD_MODE}"
echo " Kernel jobs: ${JOBS}"
echo "========================================"

sleep 1

"${ROOT_DIR}/build/scripts/userspace.sh" "${BUILD_MODE}"
"${ROOT_DIR}/build/scripts/kernel.sh" "${JOBS}"
"${ROOT_DIR}/build/scripts/initramfs.sh" "${BUILD_MODE}"
"${ROOT_DIR}/build/scripts/iso.sh"

echo
echo "Build complete"
