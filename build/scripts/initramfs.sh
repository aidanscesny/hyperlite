#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${ROOT_DIR}/out"
ROOTFS_DIR="${OUT_DIR}/rootfs"
BUILD_MODE=$1

INIT_BINARY="${ROOT_DIR}/target/x86_64-unknown-linux-musl/${BUILD_MODE}/hyperlite-init"

echo "Creating initramfs..."

rm -rf "${ROOTFS_DIR}"
mkdir -p "${ROOTFS_DIR}"

cp "${INIT_BINARY}" "${ROOTFS_DIR}/init"
chmod +x "${ROOTFS_DIR}/init"

(
    cd "${ROOTFS_DIR}"
    find . -print0 \
        | cpio --null -o --format=newc \
        | gzip -9
) > "${OUT_DIR}/initramfs.img"

echo "Created ${OUT_DIR}/initramfs.img"
