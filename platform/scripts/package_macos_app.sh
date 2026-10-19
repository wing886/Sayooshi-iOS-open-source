#!/usr/bin/env bash
set -euo pipefail


ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MACOS_DIR="${ROOT_DIR}/platform/macos/SiglusLauncher"
VENDOR_DIR="${MACOS_DIR}/Vendor"
DIST_DIR="${ROOT_DIR}/dist/macos"

# The app library package. Deliberately not SIGLUS_CARGO_PKG: CI sets that to
# siglus_scene_vm for the desktop binary builds.
LAUNCHER_CARGO_PKG="${LAUNCHER_CARGO_PKG:-game_launcher}"
RUST_LIB_NAME="${RUST_LIB_NAME:-${LAUNCHER_CARGO_PKG}}"
SIGLUS_DYLIB_NAME="${SIGLUS_DYLIB_NAME:-siglus}"
SCHEME="${SCHEME:-SiglusLauncher}"
CONFIG="${CONFIG:-Release}"
RUST_TARGET="${RUST_TARGET:-}"
OUT_APP_NAME="${OUT_APP_NAME:-Siglus.app}"

case "${RUST_TARGET}" in
  "")
    XCODE_ARCH_DEFAULT="$(uname -m)"
    ;;
  "aarch64-apple-darwin")
    XCODE_ARCH_DEFAULT="arm64"
    ;;
  "x86_64-apple-darwin")
    XCODE_ARCH_DEFAULT="x86_64"
    ;;
  *)
    echo "ERROR: Unsupported RUST_TARGET=${RUST_TARGET}"
    exit 1
    ;;
esac

XCODE_ARCH="${XCODE_ARCH:-${XCODE_ARCH_DEFAULT}}"
DERIVED_DIR="${DIST_DIR}/DerivedData-${XCODE_ARCH}"

mkdir -p "${VENDOR_DIR}" "${DIST_DIR}"

command -v cargo >/dev/null 2>&1 || { echo "ERROR: cargo not found"; exit 1; }
command -v xcodebuild >/dev/null 2>&1 || { echo "ERROR: xcodebuild not found (install Xcode)"; exit 1; }
command -v xcodegen >/dev/null 2>&1 || { echo "ERROR: xcodegen not found. Install: brew install xcodegen"; exit 1; }

echo "[macos] Building libsiglus.dylib ..."
pushd "${ROOT_DIR}" >/dev/null
CARGO_ARGS=(build --release -p "${LAUNCHER_CARGO_PKG}")
if [[ -n "${RUST_TARGET}" ]]; then
  CARGO_ARGS+=(--target "${RUST_TARGET}")
fi
cargo "${CARGO_ARGS[@]}"
popd >/dev/null

if [[ -n "${RUST_TARGET}" ]]; then
  DYLIB_PATH="${ROOT_DIR}/target/${RUST_TARGET}/release/lib${RUST_LIB_NAME}.dylib"
else
  DYLIB_PATH="${ROOT_DIR}/target/release/lib${RUST_LIB_NAME}.dylib"
fi
if [[ ! -f "${DYLIB_PATH}" ]]; then
  echo "ERROR: Missing ${DYLIB_PATH}"
  echo "Hint: ensure ${LAUNCHER_CARGO_PKG} builds a cdylib (lib${RUST_LIB_NAME}.dylib); it is packaged as lib${SIGLUS_DYLIB_NAME}.dylib."
  exit 1
fi

cp -f "${DYLIB_PATH}" "${VENDOR_DIR}/lib${SIGLUS_DYLIB_NAME}.dylib"

# Ensure a relocatable install name for rpath loading.
install_name_tool -id "@rpath/lib${SIGLUS_DYLIB_NAME}.dylib" "${VENDOR_DIR}/lib${SIGLUS_DYLIB_NAME}.dylib" || true

echo "[macos] Generating Xcode project ..."
pushd "${MACOS_DIR}" >/dev/null
xcodegen generate --spec project.yml
popd >/dev/null

XCODEPROJ="${MACOS_DIR}/${SCHEME}.xcodeproj"
[[ -d "${XCODEPROJ}" ]] || { echo "ERROR: Missing generated ${XCODEPROJ}"; exit 1; }

rm -rf "${DERIVED_DIR}"
mkdir -p "${DERIVED_DIR}"

echo "[macos] Building .app ..."
xcodebuild \
  -project "${XCODEPROJ}" \
  -scheme "${SCHEME}" \
  -configuration "${CONFIG}" \
  -derivedDataPath "${DERIVED_DIR}" \
  -arch "${XCODE_ARCH}" \
  ONLY_ACTIVE_ARCH=NO \
  build

APP_PATH="${DERIVED_DIR}/Build/Products/${CONFIG}/Siglus.app"
if [[ ! -d "${APP_PATH}" ]]; then
  # Fallback to scheme-based naming
  APP_PATH="${DERIVED_DIR}/Build/Products/${CONFIG}/${SCHEME}.app"
fi
[[ -d "${APP_PATH}" ]] || { echo "ERROR: Built app not found"; exit 1; }

FW_DIR="${APP_PATH}/Contents/Frameworks"
mkdir -p "${FW_DIR}"
cp -f "${VENDOR_DIR}/lib${SIGLUS_DYLIB_NAME}.dylib" "${FW_DIR}/lib${SIGLUS_DYLIB_NAME}.dylib"

# Ad-hoc sign for local testing. Replace '-' with a real identity for distribution.
codesign --force --sign - --timestamp=none "${FW_DIR}/lib${SIGLUS_DYLIB_NAME}.dylib" || true
codesign --force --sign - --timestamp=none --deep "${APP_PATH}" || true

OUT_APP="${DIST_DIR}/${OUT_APP_NAME}"
rm -rf "${OUT_APP}"
cp -R "${APP_PATH}" "${OUT_APP}"

echo "[macos] OK: ${OUT_APP}"
