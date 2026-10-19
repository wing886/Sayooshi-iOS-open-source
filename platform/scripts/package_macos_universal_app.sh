#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIST_DIR="${ROOT_DIR}/dist/macos"
ARM64_APP_PATH="${ARM64_APP_PATH:-${DIST_DIR}/Siglus-arm64.app}"
X86_64_APP_PATH="${X86_64_APP_PATH:-${DIST_DIR}/Siglus-x86_64.app}"
OUT_APP_PATH="${OUT_APP_PATH:-${DIST_DIR}/Siglus.app}"

[[ -d "${ARM64_APP_PATH}" ]] || { echo "ERROR: Missing ${ARM64_APP_PATH}"; exit 1; }
[[ -d "${X86_64_APP_PATH}" ]] || { echo "ERROR: Missing ${X86_64_APP_PATH}"; exit 1; }

ARM64_BIN="$(find "${ARM64_APP_PATH}/Contents/MacOS" -maxdepth 1 -type f | head -n 1)"
X86_64_BIN="$(find "${X86_64_APP_PATH}/Contents/MacOS" -maxdepth 1 -type f | head -n 1)"
[[ -n "${ARM64_BIN}" ]] || { echo "ERROR: No app executable in ${ARM64_APP_PATH}"; exit 1; }
[[ -n "${X86_64_BIN}" ]] || { echo "ERROR: No app executable in ${X86_64_APP_PATH}"; exit 1; }

ARM64_EXE_NAME="$(basename "${ARM64_BIN}")"
X86_64_EXE_NAME="$(basename "${X86_64_BIN}")"
[[ "${ARM64_EXE_NAME}" == "${X86_64_EXE_NAME}" ]] || {
  echo "ERROR: Executable name mismatch (${ARM64_EXE_NAME} vs ${X86_64_EXE_NAME})"
  exit 1
}

rm -rf "${OUT_APP_PATH}"
cp -R "${ARM64_APP_PATH}" "${OUT_APP_PATH}"

OUT_BIN="${OUT_APP_PATH}/Contents/MacOS/${ARM64_EXE_NAME}"
lipo -create "${ARM64_BIN}" "${X86_64_BIN}" -output "${OUT_BIN}"

ARM64_DYLIB="${ARM64_APP_PATH}/Contents/Frameworks/libsiglus.dylib"
X86_64_DYLIB="${X86_64_APP_PATH}/Contents/Frameworks/libsiglus.dylib"
OUT_DYLIB="${OUT_APP_PATH}/Contents/Frameworks/libsiglus.dylib"
[[ -f "${ARM64_DYLIB}" ]] || { echo "ERROR: Missing ${ARM64_DYLIB}"; exit 1; }
[[ -f "${X86_64_DYLIB}" ]] || { echo "ERROR: Missing ${X86_64_DYLIB}"; exit 1; }

lipo -create "${ARM64_DYLIB}" "${X86_64_DYLIB}" -output "${OUT_DYLIB}"

codesign --force --sign - --timestamp=none "${OUT_DYLIB}" || true
codesign --force --sign - --timestamp=none --deep "${OUT_APP_PATH}" || true

echo "[macos] Universal app created: ${OUT_APP_PATH}"
echo "[macos] Executable architectures: $(lipo -info "${OUT_BIN}")"
echo "[macos] libsiglus architectures: $(lipo -info "${OUT_DYLIB}")"
