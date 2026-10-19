# avg32

A reimplementation of VisualArt's **AVG32** engine (the pre-RealLive system
behind Kanon, AIR, ONE and many 1999–2002 titles).

| Area | Module | Notes |
| --- | --- | --- |
| Scenario bytecode (`TPC32`, `SEEN.TXT`) | `scenario`, `scene` | every command family `0x01`–`0xff`; per-frame cooperative execution; scenario menus (`smenu`) with block-relative jumps; one unified `(scene, pos)` call stack; save points |
| Variables | `flags` | 2000 values, 2000 bits, 100×64-byte strings; indirect (`0x80`) operands |
| Graphics | `buffer`, `pdtmgr` | 32 RGB+mask PDT buffers with premultiplied masks; copy / mask copy / colour-key / swap / stretch / fade / invert / mono / get-put; `-1` destinations draw without presenting |
| Transitions | `effect` | all transition patterns (dither fades, wipes, scrolls, slides, blinds, TV on/off, random lines, fan, zoom, syringe, raster wave, crossfades, …) plus the `0x13` fades, `0x68` flashes, `0x69` pans, `0x64:32` stretch tweens and `0x17` screen shakes |
| Message window | `meswin` | `#WAKUPDT` frames, multiply-tinted translucency, bracket indentation, `＊Ａ` names, `＊００` external glyphs, double-size text, click icons, novel mode (`FN.DAT`), message speed |
| Choices | `meswin` | in-window and sub-window lists, two columns, colour/disabled/hidden attributes, keyboard and mouse, decision blink, SE |
| Audio | `sound` | CD-DA (`#CDTRACK`, CloneCD images or ripped tracks) and DirectSound BGM (`#DSTRACK`, loop points), fades, WAV (pseudo-linear 8-bit), `#SE`, KOEPAC voices, volume/mute |
| Voice patches | `voicepatch` | *vair*-style `voicepat.txt` tables applied at run time (AIR + Dreamcast AFS voices) |
| Save / load | `savedata`, `system` | the original `SAVE.INI` layout for each engine revision, global flags and names, screen restoration macros, the post-load fade |
| System UI | `menu` | `#SYSCOM` right-click menu (save, load, text speed, window background and frame, volume, fast-forward, hide window, return to menu, quit), load picker, name entry, message backlog with voice replay |
| Misc | `cgmode`, `cursor`, `animation`, `movie`, `ard` | `MODE.CGM` CG gallery and slideshow, `CUR16M` cursors, `ANM32` single/multi/voice-synced animations, Cinepak AVI movies, `.ARD` click areas |

The engine revision (1604–1714) is detected from `#REGNAME` or the
executable, and selects version-dependent encodings (text position words,
copy flags, save layout, voice numbering).

## Running

```sh
cargo run -p avg32 --release --features desktop-player --bin avg32_player -- <game-root>
```

Left click / Enter advances, right click / Esc opens the system menu,
wheel-up / PageUp opens the backlog, Space hides the window, Shift or Ctrl
skips, F11 toggles full screen.

`--nls sjis|gbk|big5|utf8` (default `sjis`) selects the text encoding of
the scenario, `GAMEEXE.INI` and save data, for translated releases. Every
multi-byte character occupies one full-width cell. File names are decoded
with the same encoding and, when no such file exists, retried as Shift-JIS.

`avg32_probe` runs the engine headlessly on a simulated clock
(`--seen N`, `--shots DIR`, `--sweep` over every scene, `--nls`). The examples dump
scenes (`avg32_dump`), extract routed files (`avg32_extract`) and check a
voice patch (`avg32_voicecheck`).

Environment: `AVG32_FONT` (TrueType/OpenType font for text),
`AVG32_SAVE_DIR`, `AVG32_VERSION` (override the detected revision),
`AVG32_NO_AUDIO`, `AVG32_NLS` (encoding for the examples).

## Tests

`cargo test -p avg32` runs unit tests and hand-assembled scenarios.
Setting `AVG32_GAME_ROOT` to an installed title adds an end-to-end save/load
test against the real data.
