# Third-party notices

## Engine source

This compatibility branch is based on xmoezzz/siglus_rs at `5f358ef1360b0439e92aed2b749e8f25d8721954`, distributed under MPL-2.0. The engine modifications remain under MPL-2.0. Retain [LICENSE-MPL-2.0](LICENSE-MPL-2.0) and existing source notices.

The vendored `eluna_rs` code has its own [MPL-2.0 notice](vendor/eluna_rs/LICENSE-MPL-2.0). The Switch `unwinding` patch retains its MIT and Apache-2.0 license files. No Switch build was validated by this project.

## Embedded outline font

`crates/siglus_scene_vm/assets/fonts/default.ttf` contains the unmodified official **Noto Sans CJK SC Regular static OTF**, despite the historical filename. It is NOT the upstream Microsoft font.

Source: https://github.com/notofonts/noto-cjk/tree/f8d157532fbfaeda587e826d4cd5b21a49186f7c/Sans/OTF/SimplifiedChinese

SHA-256: `2c76254f6fc379fddfce0a7e84fb5385bb135d3e399294f6eeb6680d0365b74b`.

Copyright 2014-2021 Adobe (http://www.adobe.com/), with Reserved Font Name Source.

SIL Open Font License 1.1 applies; see the adjacent `LICENSE-NOTO.txt`. Copyright and license text also ship in iOS `Resources/ThirdPartyNotices.txt`.

The A / hiragana-a test font in `crates/avg32/tests/fixtures` is a modified subset of that same OFL source for missing-glyph regression testing. It is not a game font. Preserve its README and OFL notice.

## Other upstream assets

- `crates/uk2/assets/shinonome_jis16.bin`: identified by upstream UK2 README/source as public-domain Shinonome bitmap font; attribution retained. This is separate from user-supplied AVG32 FN.DAT.
- Icons and platform artwork are retained from the upstream MPL-2.0 source distribution. No game screenshots are added in this branch. No trademark endorsement is implied.
- Vita GXP shaders have accompanying CG/CGH source in the same upstream tree. They are not used in the iOS/Windows AVG32 validation.
- Android Gradle wrapper is upstream build tooling. Its applicable Gradle notices/Apache-2.0 terms remain relevant if separately redistributed; this project does not release an Android build.

## Excluded sample and release boundary

The upstream bundled `crates/siglus_cfx_decompiler/assets/shader.cfx` input sample (also present at `define/shader.cfx`) has no separately documented provenance in the inspected tree. It is excluded from the clean source export, including any old Git history containing it. The decompiler source remains; users can supply their own test input. This exclusion does not change the validated game runtime code.

Cargo dependencies retain their individual licenses, as recorded in their source packages/manifests. `Cargo.lock` pins dependency resolution; it does not relicense dependencies. Before a new public binary release, collect notices for the actual target's linked dependencies as well as these bundled assets. This source-preparation pass does not claim a complete transitive binary-license audit.

Commercial game files, translations, scripts, audio, artwork, FN.DAT and saved games are NOT covered by this repository's license and are not included. Redistributors must not add them to source archives, Releases, CI artifacts or Issue attachments.
