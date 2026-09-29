#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

OUT_DIR="${ROOT_DIR}/out"
ISO_ROOT="${OUT_DIR}/iso"
ISO_FILE="${OUT_DIR}/hyperlite-x86_64.iso"

echo "[+] Creating bootable ISO..."

rm -rf "${ISO_ROOT}"

mkdir -p "${ISO_ROOT}/boot/grub"

cp "${OUT_DIR}/kernel/bzImage" \
   "${ISO_ROOT}/boot/vmlinuz"

cp "${OUT_DIR}/initramfs.img" \
   "${ISO_ROOT}/boot/initramfs.img"

cp "${ROOT_DIR}/build/config/grub.cfg" \
   "${ISO_ROOT}/boot/grub/grub.cfg"

grub2-mkrescue \
    -o "${ISO_FILE}" \
    "${ISO_ROOT}"

echo "[+] ISO created: ${ISO_FILE}"
