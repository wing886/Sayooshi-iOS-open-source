# reallive

A reimplementation of VisualArt's **RealLive** engine (Kanon SE, CLANNAD,
Tomoyo After, Little Busters!, planetarian and many 2002–2009 titles).

| Area | Module | Notes |
| --- | --- | --- |
| Archives | `archive`, `scenario`, `compression` | `SEEN.TXT` plus loose `SEENnnnn.TXT` overrides; LZ + XOR decompression; second-level keys chosen by `#REGNAME` or derived statistically when unknown; RLdev encoding metadata |
| Bytecode | `bytecode`, `expr` | every element type; special flow commands (goto/gosub families, `_with`, `select`); expressions, specials and complex parameters; line markers anywhere in parameter lists |
| Machine | `machine`, `memory`, `modules/*` | banks A–F, G, Z, L, S, M, K with bit-width views; call stack with farcall/gosub arguments; cooperative long operations that can sit under a `#CANCELCALL` menu; interrupts (`SetInterrupt`/`yield`); savepoints |
| Text | `text`, `textout`, `window_buttons`, `modules/msg` | `#WINDOW` layout and frames (type 4 nine-slice, `BACK` colour masks, subtractive or blended backing), window buttons (`#WAKU…_BOX`: clear, skip, auto, backlog, `#WBCALL` extra buttons), name boxes, faces, ruby, kinsoku with punctuation squeezing, indentation after names and opening quotes, `＊Ａ`/`％Ａ` names, key cursor, message speed, auto mode, skip of read text, backlog, open/close animations |
| Choices | `select`, `modules/sel` | window choices, `#SELBTN` buttons and button objects; all five option effects (colour, title, hide, blank, cursor); keyboard, number keys and mouse; return to previous selection |
| Graphics | `graphics`, `surface`, `image`, `modules/grp` | 16 DCs, g00 (types 0/1/2) and PDT, the `grp*`/`rec*` family with masks and compositing (masked effects too), graphics stack, draw modes, screen zoom, tone curves (`.tcc`) |
| Backgrounds | `hik`, `serial_pdt`, `modules/scr`, `modules/snm` | `bgrLoadHaikei` pictures and HIK animations, scrolling (`HAIKEI_SCROLL_*`); serial animations (`snmPlay`, `snmStretch`, `snmScroll`, background slots) |
| Objects | `object`, `modules/obj` | fg/bg layers of 256 objects with children; files, GAN, text, digits, drift, filter rectangles; every property and its `objEve*` animation (with check/wait/waitC/end); `objEveDisplay` presets (fade, slide, spin, stretch, wave); range and child forms; button objects |
| Transitions | `effects` | the `#SEL` styles (fades, wipes, blinds, shapes, dithers, scrolls, squashes, slides, zooms, ripples, pixelation, masks…) |
| Shakes | `shake`, `modules/shk` | `#SHAKE` patterns, procedural and layered shakes with envelopes |
| Audio | `sound`, `modules/sound` | BGM with `#DSTRACK` loop points, 16 PCM channels, `#SE` sounds, voices from KOEPAC, NWK, OVK archives or loose files; per-character `#KOEONOFF`; fades and volume ramps; music ducking under voices |
| Movies | `movie`, `modules/mov` | MPEG-1/2 with sound, streamed from a worker thread |
| DLLs | `dll`, `pt00`, `dt00` | `EF00` (Little Busters! effects) emulated; `PT00` (Little Busters! baseball) and `DT00` (Tomoyo After's RPG) ported |
| System | `system`, `settings`, `save`, `ui`, `backlog` | settings and their defaults, system commands, save/load slots (with comments, values and thumbnails) and global data, built-in system menu, save/load lists, name entry, text input, CG table queries, RealLiveMax databases (`#DATABASE`, `.dbs`) |

Compared with rlvm and xclannad this implementation also: derives unknown
XOR keys instead of refusing to run; keeps whole scenarios NLS-aware
(Shift-JIS, GBK, Big5, UTF-8, …); stops unquoted strings before a following
special parameter (`BG001N1a\x00…`); implements every select effect,
`objEveDisplay` rotation/scale/wave and `GetCursorPos` click states;
computes whether sounds are still playing from the engine clock, so waits
behave the same with or without an audio device; plays movies; and keeps
save files pixel-exact.

## Running

```sh
cargo run -p reallive --release --features desktop-player --bin reallive_player -- <game-root>
```

Left click / Enter advances, right click / Esc opens the system menu (or the
game's own menu), wheel-up / PageUp opens the backlog, Space hides the
window, Ctrl skips, F11 toggles full screen.

Options: `--nls sjis|gbk|big5|utf8|…` (default: the encoding RLdev recorded,
else Shift-JIS; names of files that are not found are retried as
Shift-JIS), `--scene N`, `--scale N`, `--no-audio`, `--save-dir DIR`.

Environment: `REALLIVE_FONT` (TrueType/OpenType font), `REALLIVE_SAVE_DIR`,
`REALLIVE_NO_AUDIO`.

## Tools

* `rl_run <game> [frames] [--shots DIR] [--every N] [--scene N] [--click F:X:Y]`
  runs a game headlessly on a simulated clock, clicking through text, and
  lists unimplemented opcodes and errors.
* `rl_disasm <SEEN.TXT> [scene] [--quiet]` decodes scenarios.
* `rl_code <SEEN.TXT> <scene> <regname> [from] [length]` dumps bytecode.
* `rl_g00`, `rl_movie`, `rl_transitions` check images, movies and effects.

## Tests

`cargo test -p reallive` runs the unit tests. `RLVM_TEST_DATA` pointing at
rlvm's `test` directory adds its compiled scenario fixtures (not bundled:
they are GPL). `REALLIVE_TEST_GAME` pointing at an installed game adds an
end-to-end save/load test.

## DLLs

Little Busters! loads two DLLs. `EF00` (sprite effects) is emulated in
`dll.rs`; `PT00` (the baseball mini-game) is a native port of the DLL in
`src/pt00`, translated function by function from its decompilation. The
port reads the DLL's data tables from the game's own `PT00.dll` at run
time, so the file must be present in the game directory.

Tomoyo After's `DT00` ("Dungeons & Takafumi") is machine-translated from
its decompilation into `src/dt00/code` by `tools/dt00` and checked against
the original DLL run in an emulator (recorded game traces and fuzzed calls
must leave identical memory). It too reads its data from the game's
`dt00.dll`.

## Known gaps

* The settings dialogs behind some system commands (volume, message speed)
  are not drawn; scripts that provide their own menus are unaffected.
* `InvokeDLL` (calls into an arbitrary native DLL) is not supported; the
  DLLs games are known to load are.
* Switches that only change the original's own interface (hint icons,
  system button bars, the Alt menu, …) are remembered for scripts that read
  them back but change nothing on screen. A few functions are known only
  by name and signature (`STRFIND`, `KOEPLAY` window button patterns); their
  readings are marked as guesses in the code.
