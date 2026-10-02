#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="${ROOT_DIR}/out"
ROOTFS_DIR="${OUT_DIR}/rootfs"
BUILD_MODE=$1
INIT_BINARY="${ROOT_DIR}/target/x86_64-unknown-linux-musl/${BUILD_MODE}/hyperlite-init"
TEST_BINARY="${ROOT_DIR}/target/x86_64-unknown-linux-musl/${BUILD_MODE}/hyperlite-testd"

echo "[+] Creating initramfs..."

rm -rf "${ROOTFS_DIR}"
mkdir -p "${ROOTFS_DIR}"
mkdir "${ROOTFS_DIR}/proc"
mkdir "${ROOTFS_DIR}/sys"
mkdir "${ROOTFS_DIR}/dev"
mkdir -p "${ROOTFS_DIR}/system/bin"

cp "${INIT_BINARY}" "${ROOTFS_DIR}/init"
chmod +x "${ROOTFS_DIR}/init"
cp "${TEST_BINARY}" "${ROOTFS_DIR}/system/bin/"
chmod +x "${ROOTFS_DIR}/system/bin/hyperlite-testd"

(
    cd "${ROOTFS_DIR}"
    find . -print0 \
        | cpio --null -o --format=newc \
        | gzip -9
) > "${OUT_DIR}/initramfs.img"

echo "[+] Created ${OUT_DIR}/initramfs.img"