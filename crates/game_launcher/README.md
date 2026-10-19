# game_launcher

The game library and runtime behind the app launchers (macOS bundle, iOS,
Android, WebAssembly) and the `game_launcher_run` desktop runner. It hosts all
four engines of this repository:

| Engine | Detection | Runs on | Text encodings (NLS) |
|---|---|---|---|
| SiglusEngine | `Scene.pck` / `Gameexe.dat` | everywhere (own host in `siglus_scene_vm`) | — (Unicode) |
| RealLive | `Gameexe.ini` + RealLive `SEEN.TXT` / `SEEN####.TXT` | everywhere | Shift-JIS (default), GBK, Big5, UTF-8, Western, Korean, auto |
| AVG32 | `Gameexe.ini` + PACL `SEEN.TXT` / `TPC32` scenes | everywhere | Shift-JIS (default), GBK, Big5, UTF-8 |
| UK2 (PC-98) | `UK2.CFG` or UK2 `.MES` scripts | desktop, iOS, Android (not the web: it needs threads) | Shift-JIS (default), GBK, Big5, Korean |

Engine detection comes from the `engine-detect` crate. Games are found by
scanning a folder: it can be one game or a folder containing many games. The
scan goes up to a given depth, and apps import folders in batches this way.
Unsupported games (unknown engine, or UK2 on the web) are reported with a
reason, so the apps can list them after an import.

## Covers

`probe` picks a cover image for each game, in this order:

1. `cover.png` / `cover.jpg` / `thumbnail.png` / `icon.png` in the game folder
2. a `.ico` file in the game folder
3. the icon embedded in the game's `.exe` (PE `RT_GROUP_ICON`)
4. a picture from the game itself (for example, UK2 title artwork decoded from PDT files)

Icons are placed on a 16:9 card and scaled with nearest-neighbour filtering;
`cover_kind` is `icon` or `image`, so UIs can keep pixel art crisp.

## C API (`siglus.h`)

* `game_scan_json(path, depth, cover_cache_dir)`: JSON array of probe results.
* `game_probe_json(root, nls, cover_cache_dir)`: one probe result. Its fields
  are `id`, `root`, `engine`, `engine_name`, `supported`, `unsupported_reason`,
  `title`, `cover`, `cover_kind`, `nls` and `nls_options[{id,label}]`.
* `game_fb_open(root, engine, nls, &err)` opens a RealLive, AVG32 or UK2 game.
  Each frame, call `game_fb_step(h, dt_ms)` (0 = running, 1 = ended), then read
  the RGBA frame with `game_fb_frame`. Send input through
  `game_fb_pointer_move/button/wheel`, `game_fb_key` and `game_fb_text`, and
  close the game with `game_fb_close`.
* `game_run_entry(root, nls)` runs any game in a desktop window (macOS bundle).
* `game_add_font_file(path)` registers an extra font for game text.

Key codes: 1 Enter, 2 Escape, 3 Space, 4–7 arrows (up, down, left, right),
8/9 PageUp/PageDown, 10/11 Home/End, 12 Backspace, 13 Tab, 14 Ctrl, 15 Shift,
`0x100 + n` for F*n*, and `0x10000 + codepoint` for a character key.

## Web

`platform/wasm` imports `pkg/game_launcher.js`, which `build_wasm.sh`
produces:

* Import or drop several folders; all of them are indexed under their names and
  scanned with `launcherScan`.
* SiglusEngine games start through `start_siglus_from_directory`; RealLive and
  AVG32 games run in `WebGame` and are drawn to the canvas.
* Save files written by games are kept in IndexedDB.
* Browsers expose no system fonts. Add a CJK font with *Add Font* (it is
  remembered), or ship fonts next to the page and list their file names in
  `fonts/fonts.json`.

## Desktop runner

```bash
cargo run --release -p game_launcher --bin game_launcher_run -- <game-dir> [sjis|gbk|big5|utf8|korean]
cargo run --release -p game_launcher --bin game_launcher_run -- --probe <folder>...
```
