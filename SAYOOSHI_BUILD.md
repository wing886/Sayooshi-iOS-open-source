# Compatibility branch provenance

Upstream: https://github.com/xmoezzz/siglus_rs

Pinned upstream commit: `5f358ef1360b0439e92aed2b749e8f25d8721954`.
Imported from the official source archive (SHA-256 `bc624ffaa5ac53ed76cdd63560b3e24f3cb992e1bf464564f7e36c7fcffa0630`); this checkout does not claim upstream Git history.

The tested engine is commit `69462ec26829dca9e26b01fd76a8a6a3f6c8a5bf`, IPA 1.0.1 (2). See [TESTING.md](TESTING.md) for current evidence and limitations, and [BUILDING.md](BUILDING.md) for commands.

## Runtime changes

- Preserve raw Shift-JIS font slots in novel text and choices, including slots without Unicode mappings.
- Draw novel history using the same user-supplied FN.DAT glyph mapping, with bounded layout.
- Implement AVG32 0x76:03 (clear history), :04 (enable), :05 (disable). Behavior was checked against local original executable behavior/history buffer handling. Unknown instructions are not silently discarded.
- Add opt-in TITLE_NLS for mixed-encoding script titles, independent of body decoding.
- Replace the upstream embedded outline font with unmodified official Noto Sans CJK SC Regular static OTF, add missing-glyph fallback and regression tests.
- Add resource-free regression tests, a local-installation acceptance driver, locked device/simulator builds, unsigned packaging and commit provenance.

## Distribution boundary

MPL-2.0 notices are retained. Fonts and other exceptions retain their own licenses; see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md). Game assets, Chinese overlay resources, recovered game scripts and private acceptance evidence are not repository contents. No game font is embedded in the engine.

Cloud builds use source code only. They do not validate a complete commercial game, and require no Apple account credentials. The workflow is manual-only; its owner must check cost settings before dispatch.
