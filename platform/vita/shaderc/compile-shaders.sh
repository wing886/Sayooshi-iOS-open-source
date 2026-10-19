#!/bin/zsh
# Compiles the Vita renderer's Cg shaders to GXP in Vita3K and copies them
# into the source tree (crates/siglus_scene_vm/src/render/vita/shaders/gxp),
# which the player embeds.
#
# Needs VitaSDK, cargo-vita and Vita3K with Sony's libshacccg.suprx in its
# ur0:data. Vita3K boots the app only while its window is visible.
set -e
ROOT=${0:A:h:h:h:h}
VITA3K=${VITA3K:-/Applications/Vita3K.app/Contents/MacOS/Vita3K}
FS=${VITA3K_FS:-"$HOME/Library/Application Support/Vita3K/Vita3K/fs"}
SHADERS=$ROOT/crates/siglus_scene_vm/src/render/vita/shaders

cd $ROOT/platform/vita/shaderc
VITASDK=${VITASDK:-/usr/local/vitasdk} cargo +nightly vita build vpk -- --release
APP=$FS/ux0/app/SIGSHADR1
OUT=$FS/ux0/data/siglus_shaderc
rm -rf $APP $OUT
mkdir -p $APP
unzip -q -o target/armv7-sony-vita-newlibeabihf/release/siglus_vita_shaderc.vpk -d $APP

pkill -9 -f Vita3K || true
sleep 1
$VITA3K -r SIGSHADR1 > /dev/null 2>&1 &
for i in {1..120}; do
    grep -q DONE $OUT/report.txt 2>/dev/null && break
    sleep 1
done
pkill -9 -f Vita3K || true
grep -v 'code 730[12]' $OUT/report.txt
grep -q 'DONE failed=0' $OUT/report.txt || { echo "shader compilation failed"; exit 1; }
rm -f $SHADERS/gxp/*.gxp
mkdir -p $SHADERS/gxp
cp $OUT/*.gxp $SHADERS/gxp/
echo "copied $(ls $SHADERS/gxp | wc -l | tr -d ' ') programs to $SHADERS/gxp"
