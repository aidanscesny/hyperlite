#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

source "${ROOT_DIR}/build/config/versions.env"

OUT_DIR="${ROOT_DIR}/out"
DOWNLOAD_DIR="${OUT_DIR}/downloads"
SRC_DIR="${OUT_DIR}/src"
KERNEL_SRC="${SRC_DIR}/linux-${LINUX_VERSION}"
COMPILE_CORES=$1

KERNEL_ARCHIVE="linux-${LINUX_VERSION}.tar.xz"
KERNEL_URL="https://cdn.kernel.org/pub/linux/kernel/v6.x/${KERNEL_ARCHIVE}"

mkdir -p "${DOWNLOAD_DIR}" "${SRC_DIR}" "${OUT_DIR}/kernel"

if [[ ! -f "${DOWNLOAD_DIR}/${KERNEL_ARCHIVE}" ]]; then
    echo "[+] Downloading Linux ${LINUX_VERSION}..."
    curl -L \
        "${KERNEL_URL}" \
        -o "${DOWNLOAD_DIR}/${KERNEL_ARCHIVE}"
fi

if [[ ! -d "${KERNEL_SRC}" ]]; then
    echo "[+] Extracting Linux ${LINUX_VERSION}..."

    tar -xf "${DOWNLOAD_DIR}/${KERNEL_ARCHIVE}" \
        -C "${SRC_DIR}"
fi

echo "[+] Configuring Linux ${LINUX_VERSION}..."

cp "${ROOT_DIR}/build/config/kernel.config" \
   "${KERNEL_SRC}/.config"

make -C "${KERNEL_SRC}" olddefconfig

echo "[+] Building Linux ${LINUX_VERSION}..."

make -C "${KERNEL_SRC}" -j"${COMPILE_CORES}"

cp "${KERNEL_SRC}/arch/x86/boot/bzImage" \
   "${OUT_DIR}/kernel/bzImage"

echo "[+] Kernel built: ${OUT_DIR}/kernel/bzImage"
