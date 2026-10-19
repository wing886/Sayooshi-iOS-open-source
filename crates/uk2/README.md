# uk2

`uk2` implements AyPio's early UK2 engine (`UK2.EXE`, "SOSAR SYSTEM"), using
the PC-98 version of *Sorcer Kingdom* as the reference game.

UK2 is not routed through AVG32: it has its own MES interpreter, operand
model, recursive status protocol, DLB archive format and the older PDT34
graphics family.

## Engine port (`uk2::engine`)

`uk2::engine::Engine` is a port of `UK2.EXE` itself. The interpreter,
window system, text renderer, maps, actors and battles work on the same state
the original keeps in its data segment, so they behave like the executable
instead of like a reimplementation built from observed script behaviour:

- **Memory** (`mem`): the 64 KiB data segment is loaded from the game's own
  `UK2.EXE` image (its initialised tables, strings and defaults), and
  every `farmalloc` block gets its own segment. Far pointers keep their
  original `seg:off` encoding, so pointer-valued MES operands and the save
  files match the original layout.
- **Graphics** (`vram`, `gfx`): two pages of four PC-98 bit planes, the
  analog `0xGRB` palette with `COLOR.TBL` banks and fades, the PDT34 decoder,
  16x16 cell chips, sprite merging and the dissolve transitions.
- **Objects and text** (`object`, `text`, `ui`): the 70-entry object table,
  window composition on page 0 over the page-1 background using the cell
  ownership maps, frames and gadgets, `U2`/`D0` formatted text with
  its escapes, text speed and scrolling, `W8` menus, mouse hit testing and
  window dragging.
- **Maps and actors** (`map`): `F*` map loading, viewport, triggers,
  collisions, entity AI and movement, chip views and the overlay chips.
- **Battles** (`battle`): the `I*` commands with `FIGHT.TAB` and
  `LEVEL.TAB`.
- **System** (`system`): the start-up sequence, the top-level `MES` loop,
  save/load (`U6`/`U7`/`UJ`, `flagNN.dat` files) and the remaining service
  commands.
- **Input and timing** (`mod.rs`): the VSYNC (56.4 Hz), mouse and keyboard
  interrupt handlers, with PC-98 scan codes and the RETURN/SPACE/ESC mouse
  button aliases.
- **Fonts** (`font`): kanji come from a real `KANJI16.ROM`
  (`UK2_KANJI_ROM`), if you have one, or from the embedded public-domain
  Shinonome 16-dot JIS font. Half-width characters are captured from the
  game's own `kana.pdt` at start-up, like the original does.
- **Music** (`pmd`, `opna`, `audio`): a port of KAJA's PMD v4.8 driver
  (FM parts A-F, SSG parts G-I, rhythm part K) that plays the game's `.MMM`
  scores on a YM2608 (PC-9801-86) model. The OPNA rhythm ROM is not
  available, so the six rhythm voices are synthesised. The engine reports
  only the FM driver to the game, so it chooses `.MMM` over the MIDI `.MMD`
  scores.

The engine runs on its own thread and talks to the host through the
`Platform` trait: VSYNC ticks, input events, finished frames and music
commands.

## Format layer

The older format modules are still available and are used by `uk2_verify`:

- `<< UK2 TEXT Ver1.00 >>` MES validation, the full opcode table and a
  control-flow-aware disassembler (checked against all 552 supplied MES
  files);
- `UK2.CFG` parsing and virtual extension mapping (`.mes1` -> `.MES`, etc.);
- DLB Ver1.00 archives, PDT34 images, MAP files, and the `.MMM`/`.MMD`
  directories.

## Binaries

```text
uk2_player <game-directory>                 desktop player (wgpu + winit + kira)
uk2_run <game-directory> [--script FILE] [--out DIR] [--ticks N] [--trace|--trace-mes] [--start NAME]
uk2_music <score.MMM> <out.wav> [seconds]   render a PMD score to WAV
uk2_disasm <file.MES>
uk2_verify <game-directory> [--assets|--boot-menu|--boot-path=N|--stats]
```

`uk2_player` maps the keyboard to PC-98 scan codes (arrow keys and the
numeric keypad move, Enter/Space act as the left button and Esc as the right
button) and passes the mouse through as an absolute pointer. The engine
draws the game's own software cursor.

`uk2_run` runs the engine headless on a virtual VSYNC clock. A script lists
timed input, one command per line: `<tick> move X Y`, `click X Y`,
`rclick X Y`, `down`/`up`/`rdown`/`rup`, `key NAME`, `keydown NAME`,
`keyup NAME`, and `shot FILE.png`. At the end it writes `final.png` and prints
the engine state (palette, objects, text).

See `UK2_REVERSE_ENGINEERING.md` for the recovered formats and `UK2.EXE`
routines.
