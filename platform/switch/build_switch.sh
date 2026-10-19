#!/bin/sh
# Switch build entry: builds the existing Rust engine plus its libnx/deko3d
# Horizon shell.  It never selects winit or wgpu for this target.
# Requires devkitPro (DEVKITPRO=/opt/devkitpro).
set -e
export DEVKITPRO=/opt/devkitpro
export DEVKITA64=/opt/devkitpro/devkitA64
export PATH="/opt/devkitpro/devkitA64/bin:/opt/devkitpro/tools/bin:$PATH"
cd "$(dirname "$0")/runtime"
make "$@"
ls -lh siglus_switch.nro
