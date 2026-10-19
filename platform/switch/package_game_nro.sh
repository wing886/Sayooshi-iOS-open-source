#!/bin/sh
# Build either a self-contained NRO or an SD-card deployment bundle without
# modifying the source game directory.
set -eu

mode=romfs
case "${1:-}" in
    --romfs)
        shift
        ;;
    --sdmc)
        mode=sdmc
        shift
        ;;
esac

if [ "$#" -lt 1 ] || [ "$#" -gt 2 ]; then
    echo "Usage: $0 [--romfs|--sdmc] GAME_DIRECTORY [OUTPUT]" >&2
    exit 64
fi

script_dir=$(CDPATH= cd "$(dirname "$0")" && pwd)
game_dir=$(CDPATH= cd "$1" && pwd)

if [ ! -f "$game_dir/Scene.pck" ]; then
    echo "Game directory must contain Scene.pck: $game_dir" >&2
    exit 66
fi

if [ "$#" -eq 2 ]; then
    output_path=$2
elif [ "$mode" = sdmc ]; then
    output_path="$script_dir/dist/siglus_rs"
else
    output_path="$script_dir/runtime/siglus_switch-$(basename "$game_dir").nro"
fi

case "$output_path" in
    /*) ;;
    *) output_path="$(pwd)/$output_path" ;;
esac

if [ "$mode" = sdmc ] && [ -e "$output_path" ]; then
    echo "SDMC bundle destination already exists: $output_path" >&2
    exit 73
fi
mkdir -p "$(dirname "$output_path")"

# The Makefile accepts ROMFS as an override.  A unique staging tree means this
# operation neither changes the source game nor erases an existing local RomFS.
stage_dir=$(mktemp -d /tmp/siglus-switch-romfs.XXXXXX)
cleanup() {
    rm -rf "$stage_dir"
}
trap cleanup EXIT HUP INT TERM

mkdir -p "$stage_dir/game"
if [ "$mode" = romfs ]; then
    cp -a "$game_dir/." "$stage_dir/game/"
fi

"$script_dir/build_switch.sh" -B -j1 "ROMFS=$stage_dir"
if [ "$mode" = romfs ]; then
    cp "$script_dir/runtime/siglus_switch.nro" "$output_path"
    echo "Embedded game: $game_dir"
    echo "Standalone NRO: $output_path"
    ls -lh "$output_path"
else
    mkdir "$output_path"
    cp "$script_dir/runtime/siglus_switch.nro" "$output_path/siglus_switch.nro"
    cp -a "$game_dir" "$output_path/game"
    echo "SDMC bundle: $output_path"
    echo "Copy its contents to sdmc:/switch/siglus_rs/"
    ls -lh "$output_path/siglus_switch.nro"
fi
