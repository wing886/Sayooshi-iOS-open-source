# UK2 reverse-engineering specification (Sorcer Kingdom reference)

This document records the bytecode and file-format facts recovered directly from the PC-98 `UK2.EXE` and `UK2.EXE.asm` supplied with Sorcer Kingdom. It intentionally separates confirmed behavior from still-unnamed engine services.

## Reference and validation

- Reference interpreter: `UK2.EXE`; IDA listing: `UK2.EXE.asm`.
- MES interpreter: `sub_19640`. Byte fetch: `sub_1ED9A`.
- Two-character opcode value table: `word_1A60A`; handler table: `jpt_19849`.
- Single-byte command table: `word_1A76A`.
- Operand decoder: `sub_14FFA`; expression decoder: `sub_15222`; condition decoder: `sub_1542C`.
- All 552 `.MES` files in the supplied game begin with the same 23-byte magic and were control-flow decoded with the grammar below. After accounting for parameters parsed inside `sub_16F89` (`D0`/`U2`) and `sub_1B068` (`W8`), validation is **552/552 files, zero decode failures**.

## MES container

```text
offset  size  meaning
0x0000  0x17  "<< UK2 TEXT Ver1.00 >>\0"
0x0017  ...   bytecode entry point
```

`sub_15595` explicitly sets `word_271A6 = 0x17` after loading a called MES. `word_271A6` is the current file-relative program counter, `dword_28EC4` is the current MES base, and `word_271A8` is its length.

## Operand encoding (`sub_14FFA`)

The first operand byte is split as `class = byte >> 5` and `index = byte & 0x1f`.

| class | encoding / result |
|---:|---|
| 0 | index 0..30: word at DS:3B62 + index*2; index 31: consume u16 LE immediate, return word scratch |
| 1 | string slot at DS:5312 + index*0x3e |
| 2 | word at current local-table base + index*2 |
| 3 | inline string until `00`; marker `05 slot` inserts current-record 0x15-byte string slot, marker `06 slot` inserts global DS:5A12 0x15-byte slot |
| 4 | consume nested expression; result must be <=0x100; selector 5 indexes DS:57B4 and selector 8 indexes DS:56B4; result type is byte |
| 5 | consume one selector byte indexing the far-pointer table at DS:2B8/2BA; low index 0=byte, 1=word, 2=string |
| 6 | 0x7d2-byte string/buffer slot at DS:3B9C + index*0x7d2 |

Class 3 uses one-based insertion slot numbers (`slot - 1` in the executable). The parser raises an engine error if the assembled inline buffer reaches 0x7d2 bytes.

## Expression encoding (`sub_15222`)

Each term starts with one control byte. Bit 7 marks the last term. Low 7 bits select the operation: `0` = null expression marker, `1` add/concatenate, `2` subtract, `3` multiply, `4` divide, `5` modulo, `6` invalid/error, `7` set/direct. A nonzero operation is followed by one operand. Numeric results are clamped at expression end to 0..0xff or 0..0xffff according to the last operand width. Negative numeric results clamp to zero. String operation 7 copies and operation 1 concatenates.

## Conditions (`sub_1542C`)

```text
<expression lhs> <compare:u8> <expression rhs> <join:u8> ...

compare: 1 <, 2 >, 3 ==, 4 !=
join:    0 end, 1 AND, 2 OR
```

The accumulator begins false and the first clause is ORed in. String operands are compared through the engine string comparator; numeric operands use unsigned 16-bit comparisons.

## Single-byte commands

| byte | behavior |
|---|---|
| `00` | return status 0 from current recursive interpreter block |
| `01` | assignment: direct destination + expression; byte destinations clamp to 0xff, word destinations to 0xffff; string destinations use the original 0x3e / 0x7d2 size limits |
| `02` | consume raw O16 target, recursively execute inline body, restore condition flag, then set PC=target; nonzero body status propagates |
| `03` | saturating decrement of direct numeric destination |
| `04` | saturating increment of direct numeric destination |
| `R` | consume next byte and return `byte - '0'` as interpreter status |

## Opcode signature notation

`D` = direct operand; `E` = required expression; `X<=N` = expression list terminated by null expression with N-value bound; `X*` = null-terminated expression list without an explicit bound in that handler; `C` = condition; `O16` = raw little-endian file-relative target; `RESOURCE_LIST` = repeated `E(resource), X<=2` entries terminated by a null resource expression.

## Two-character opcode table

| opcode | handler | encoding | reachable uses in 552 MES | confirmed behavior |
|---|---:|---|---:|---|
| `A0` | `0x1A4BB` | `D` | 0 | begin/configure buffered render/effect transaction (`sub_14BA9`) |
| `D0` | `0x19C4E` | `E X<=30` | 0 | calls the same `sub_16F89` parser with argument 1 |
| `F0` | `0x1A0DF` | `D` | 61 | loads a MAP scene through `sub_1CC8D`, including tile data, entities, triggers, and referenced graphics |
| `I0` | `0x1A561` | `D X<=8` | 1 | handler-specific engine service; exact low-level implementation at `loc_1A561` (not renamed without evidence) |
| `J0` | `0x1984E` | `O16` | 0 | absolute jump: `PC = O16` |
| `L0` | `0x198ED` | `C O16` | 375 | condition-controlled loop; body is the bytecode immediately after the O16 field |
| `M0` | `0x1A470` | `D` | 4 | music resource command via `sub_22AB8` |
| `T0` | `0x1A049` | `X*` | 21 | copy null-terminated expression strings into the current MES record table (`dword_29076`, 0x15-byte slots) |
| `U0` | `0x19D04` | `E` | 968 | show/hide the mouse cursor through `sub_21A38`; the helper's previous mode is in AX and does not become interpreter status |
| `A1` | `0x1A4D3` | `—` | 0 | close/restore buffered render/effect transaction (`sub_14C44`) |
| `D1` | `0x19C53` | `E E` | 0 | handler-specific engine service; exact low-level implementation at `loc_19C53` (not renamed without evidence) |
| `F1` | `0x1A0F7` | `E D` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A0F7` (not renamed without evidence) |
| `I1` | `0x1A594` | `—` | 1 | handler-specific engine service; exact low-level implementation at `loc_1A594` (not renamed without evidence) |
| `J1` | `0x19866` | `O16` | 9347 | local MES subroutine call; status 2 is normal return, >2 propagates |
| `L1` | `0x198F4` | `C O16` | 1528 | if block; false jumps to O16, true recursively executes inline body |
| `M1` | `0x1A484` | `X<=1` | 139 | set music driver volume through `sub_22C40`; signed values outside 0..127 are replaced with 127 |
| `T1` | `0x1A0A3` | `X*` | 0 | copy null-terminated expression strings into global 0x15-byte slots at DS:5A12 |
| `U1` | `0x19D14` | `X<=2` | 689 | palette transition: first operand is per-step wait count, second is a one-based `COLOR.TBL` bank; the executable interpolates 12-bit RGB nibbles in up to 16 steps |
| `W1` | `0x19927` | `X<=7` | 15 | define a window from seven values: left, top, right, bottom, two attributes, object ID; `sub_1669B` scales horizontal coordinates by 16 pixels and vertical coordinates by 16 pixels |
| `A2` | `0x1A4DB` | `D D D D D D` | 0 | register six-direct-operand rectangle/region descriptor (`sub_14C8B`) |
| `I2` | `0x1A59B` | `—` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A59B` (not renamed without evidence) |
| `J2` | `0x198A8` | `D` | 1131 | call another MES; callee starts at 0x17 with fresh local table; status 3 is normal return |
| `L2` | `0x198FB` | `O16` | 39 | else block controlled by prior condition flag |
| `M2` | `0x1A49D` | `—` | 2 | stop music through driver function 1 (`sub_22C1F`) |
| `U2` | `0x19D30` | `E X<=30` | 435 | calls `sub_16F89(0)`; helper itself consumes E + up to 30 further expressions |
| `W2` | `0x19955` | `—` | 1 | waits for a key or left-button event (`sub_213B6`), then waits for release and clears input counters (`sub_213D4`) |
| `A3` | `0x1A506` | `X<=20` | 0 | configure render/effect descriptor bytes (`sub_14D4C`) |
| `I3` | `0x1A5A5` | `D X<=1` | 1 | handler-specific engine service; exact low-level implementation at `loc_1A5A5` (not renamed without evidence) |
| `J3` | `0x198DD` | `D` | 0 | store/format target name and return interpreter status 5 |
| `L3` | `0x19902` | `D E E E O16` | 80 | inclusive counted loop: destination,start,end,step,exit target |
| `M3` | `0x1A5F0` | `—` | 0 | no-op in this executable (jump table lands on normal continuation) |
| `U3` | `0x19D40` | `X<=2 D` | 3 | displays a named image through `sub_22952` after setting display mode and transition values; the Rust player currently shows the image without the original transition timing |
| `W3` | `0x19962` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_19962` (not renamed without evidence) |
| `A4` | `0x1A523` | `X<=20` | 0 | configure render/effect descriptor bytes (`sub_14DCF`) |
| `F4` | `0x1A119` | `D D D E D D` | 2 | handler-specific engine service; exact low-level implementation at `loc_1A119` (not renamed without evidence) |
| `I4` | `0x1A5C9` | `—` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A5C9` (not renamed without evidence) |
| `L4` | `0x19909` | `C O16` | 2366 | conditional inline block; taken arm terminates the current interpreter block normally |
| `M4` | `0x1A4A5` | `—` | 1 | start/restart the currently loaded score through driver function 0 (`sub_22BFE`) |
| `W4` | `0x1997F` | `X<=2` | 1123 | handler-specific engine service; exact low-level implementation at `loc_1997F` (not renamed without evidence) |
| `A5` | `0x1A544` | `D` | 0 | select render/effect descriptor (`sub_14E20`) |
| `F5` | `0x1A182` | `E` | 61 | attaches the most recently loaded MAP to the given object ID through `sub_1CF65` |
| `I5` | `0x1A5D0` | `D` | 58 | writes 1 if `sub_17E5F` finds a nearby active actor with ID `0xC8..0xD6`, otherwise 0; also sets interaction ID and status globals on a hit; requires the live actor table, which the Rust player has not yet ported |
| `L5` | `0x1991D` | `C O16` | 0 | else-if block; prior true arm evaluates condition then skips to O16 |
| `M5` | `0x1A4AD` | `X<=2` | 0 | two-value music path; joins the M-family shared handler |
| `U5` | `0x19DA2` | `D E` | 1 | handler-specific engine service; exact low-level implementation at `loc_19DA2` (not renamed without evidence) |
| `W5` | `0x199BE` | `D E` | 643 | PDT/image load: direct filename + expression mode; mode stored in `word_29000` before `sub_1EB9E` |
| `A6` | `0x1A559` | `—` | 0 | execute buffered render/effect sequence (`sub_14E6E`) |
| `F6` | `0x1A1A4` | `D` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A1A4` (not renamed without evidence) |
| `U6` | `0x19DDB` | `E E` | 2 | handler-specific engine service; exact low-level implementation at `loc_19DDB` (not renamed without evidence) |
| `W6` | `0x199E1` | `E D E` | 393 | create/update a text-window object from a direct string operand (`sub_1E90B`); all supplied calls use the fixed-string dialogue slot |
| `F7` | `0x1A1B9` | `D` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A1B9` (not renamed without evidence) |
| `U7` | `0x19E25` | `E` | 3 | handler-specific engine service; exact low-level implementation at `loc_19E25` (not renamed without evidence) |
| `W7` | `0x19A17` | `E` | 407 | set the UI object that gates interpreter progress until user input; the desktop host waits for a fresh confirm/cancel press |
| `F8` | `0x1A1CE` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A1CE` (not renamed without evidence) |
| `U8` | `0x19E5F` | `X<=2` | 0 | handler-specific engine service; exact low-level implementation at `loc_19E5F` (not renamed without evidence) |
| `W8` | `0x19A29` | `D E E E X<=20` | 15 | menu selector through `sub_1B068`/`sub_1AAF9`; the three E values are object ID, width in 16-pixel units, and row-style flag; writes the selected one-based source index to D |
| `F9` | `0x1A1F6` | `X<=3` | 20 | handler-specific engine service; exact low-level implementation at `loc_1A1F6` (not renamed without evidence) |
| `U9` | `0x19E8D` | `D` | 0 | write `(sub_21339() & 1)` to direct destination |
| `W9` | `0x19A31` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_19A31` (not renamed without evidence) |
| `FA` | `0x1A244` | `X<=5` | 427 | updates an entity by ID through `sub_1BD89`; x, y, and byte attribute accept 1000 as unchanged, low attribute nibble is replaced and flag 1 toggled |
| `WA` | `0x19A7A` | `E` | 590 | remove the UI object with the supplied ID through `sub_1EA69` |
| `FB` | `0x1A25F` | `X<=5` | 147 | updates one map cell through `sub_1C2E3`: x, y, object ID, chip index, layer; values above 511/3 preserve the respective existing field |
| `UB` | `0x19EA4` | `D D` | 0 | file-open/existence test (`rb`); writes 0 on success and 1 on failure to first direct destination |
| `WB` | `0x19A94` | `X<=2` | 9 | handler-specific engine service; exact low-level implementation at `loc_19A94` (not renamed without evidence) |
| `FC` | `0x1A2A2` | `X<=2` | 7 | clears byte +8 of the matching map trigger (object ID, trigger ID) |
| `WC` | `0x19A55` | `E` | 125 | finds the object by ID through `sub_1E70B` and redraws it with `sub_15E4B` |
| `FD` | `0x1A2A2` | `X<=2` | 29 | sets byte +8 of the matching map trigger (object ID, trigger ID) |
| `UD` | `0x19F1B` | `E D` | 0 | handler-specific engine service; exact low-level implementation at `loc_19F1B` (not renamed without evidence) |
| `WD` | `0x19ACF` | `E` | 202 | handler-specific engine service; exact low-level implementation at `loc_19ACF` (not renamed without evidence) |
| `FE` | `0x1A330` | `D` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A330` (not renamed without evidence) |
| `UE` | `0x19F4E` | `E RESOURCE_LIST` | 212 | handler-specific engine service; exact low-level implementation at `loc_19F4E` (not renamed without evidence) |
| `WE` | `0x19B12` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_19B12` (not renamed without evidence) |
| `WF` | `0x19B22` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_19B22` (not renamed without evidence) |
| `FG` | `0x1A384` | `D` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A384` (not renamed without evidence) |
| `WG` | `0x19B3F` | `—` | 0 | handler-specific engine service; exact low-level implementation at `loc_19B3F` (not renamed without evidence) |
| `FH` | `0x1A3A5` | `X<=2 D D D D D` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A3A5` (not renamed without evidence) |
| `UH` | `0x19FAA` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_19FAA` (not renamed without evidence) |
| `WH` | `0x19B66` | `X<=5` | 0 | handler-specific engine service; exact low-level implementation at `loc_19B66` (not renamed without evidence) |
| `FI` | `0x1A40F` | `D` | 80 | restores the backing image over columns 6..69 and scanlines 16..239, then loads and displays the named PDT through `sub_1C0B0`; Rust host now restores its scene base and composites the PDT at its encoded rectangle |
| `UI` | `0x19FD3` | `X<=6` | 0 | handler-specific engine service; exact low-level implementation at `loc_19FD3` (not renamed without evidence) |
| `WI` | `0x19BB7` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_19BB7` (not renamed without evidence) |
| `FJ` | `0x1A424` | `X<=3` | 36 | centers and clamps the MAP viewport at x, y for an object through `sub_1CEC9` |
| `UJ` | `0x19FFB` | `E D` | 8 | handler-specific engine service; exact low-level implementation at `loc_19FFB` (not renamed without evidence) |
| `WJ` | `0x19BCF` | `D E D` | 0 | handler-specific engine service; exact low-level implementation at `loc_19BCF` (not renamed without evidence) |
| `FK` | `0x1A458` | `—` | 202 | runs the MAP actor/input update loop in `sub_1DB80`; the Rust player currently supplies button down event counters and a frame tick, while actor movement and collision remain to port |
| `UK` | `0x1A034` | `E` | 0 | handler-specific engine service; exact low-level implementation at `loc_1A034` (not renamed without evidence) |
| `WK` | `0x19BFF` | `X<=6` | 0 | handler-specific engine service; exact low-level implementation at `loc_19BFF` (not renamed without evidence) |
| `WL` | `0x19C22` | `X<=4` | 0 | handler-specific engine service; exact low-level implementation at `loc_19C22` (not renamed without evidence) |

The reachability count is a property of this Sorcer Kingdom data set, not proof that an opcode is absent from other UK2 revisions. Several valid handlers are unused by this title.

## Control-flow status protocol

The interpreter status range is 0..7. `J1` requires a local callee to return at least 2 and consumes status 2 as normal return. `J2` requires an external MES callee to return at least 3 and consumes status 3 as normal return; status 4/5 propagates. `J3` returns status 5. Loop helpers treat status 1 as a break-like condition and status 6 as a continue/reset path according to their individual handlers. This status protocol is why the Rust VM must preserve recursive interpreter structure instead of flattening branches into a conventional PC loop.

## External MES call context (`sub_15595`)

On `J2`, the executable saves the current MES base/PC/length and current record pointer, loads the new MES, sets PC to 0x17, replaces the class-2 local table with a fresh stack table, replaces the current record with a fresh record, recursively invokes `sub_19640`, then restores the parent context. The Rust implementation mirrors this local-context reset rather than sharing class-2 locals across files.

## DLB archive

```text
0x16 bytes  "<< dlb file Ver1.00>>\0"
u16 LE       file count N
N * 0x15     entries

entry:
0x0d bytes   NUL-terminated Shift-JIS filename
u32 LE       absolute file offset
 u32 LE       file length
```

`Uk2Game` indexes top-level `.DLB` files at open and reads an archive entry
when a requested resource is not present as a standalone file. Virtual
extensions (for example `.mes1`) are normalized before both lookups. MAP
discovery and base/overlay image reads use the same archive-aware path.

## Music resource status

`UK2.EXE` talks to two resident KAJA drivers, both shipped with the game:
`MMD.COM` (MIDI, `int 61h`) and `PMD.COM` v4.8e (FM, `int 60h`). At start-up
`word_25DC6` records which are present (bit 0 = MMD, bit 1 = PMD); MMD wins
when both are loaded. `M0` (`sub_22AB8`) swaps the name's extension for
`UK2.CFG`'s `MIDI_EXT` or `FM_EXT`, loads the score and starts it (driver
function 0). `M1` (`sub_22C40`) is driver function 2, **fade-out**, with the
speed clamped to 0..127. `M2` (`sub_22C1F`) is function 1 (stop) and `M4`
(`sub_22BFE`) is function 0 (start).

`.MMM` files are compiled PMD "M" data for the YM2608 (the PC-9801-86 board):
one leading byte, then (relative to byte 1) eleven little-endian part offsets
A-K, the R-pattern table offset and, because the first offset is not 24, the
offset of the embedded FM voice table (26-byte records: voice number, DT/ML,
TL, KS/AR, AM/DR, SR, SL/RR in slot order 1,3,2,4, then FB/ALG). Parts D-F are
the second FM port, G-I the SSG and K the rhythm part. The supplied K parts
only set rhythm levels and pans; the drums are triggered with the rhythm-key
command (`EBh`) inside the other parts.

The engine port includes a PMD driver port (`engine::pmd`) and a YM2608 model
(`engine::opna`) and reports only the FM driver, so the game picks `.MMM`.
The `.MMD` reader validates the 18-entry directory of all 31 SK `.MMD` files
(little-endian stream offset, key byte, MIDI channel byte with `0xff` for
the control streams; tempo and key at bytes 0/1, title at `0x50`). MMD
playback is not implemented.

## MAP resource

The 226 supplied `.MAP` files use this observed header:

```text
0x00  u16 LE   map width
0x02  u16 LE   map height
0x04  16 bytes NUL-terminated Shift-JIS base PDT name
0x14  16 bytes NUL-terminated Shift-JIS overlay PDT name
0x24  u16 LE   tile-grid byte count (observed as width * height * 2)
0x26  u8       entity count
0x27  u8       map metadata (meaning still under investigation)
0x28  u8       trigger count
0x29  17 bytes other per-map header fields (preserved)
0x3a  u16 LE[] row-major tile ids, one value per map cell
...            entity count × 17-byte entity records
...            trigger count × 10-byte trigger records
```

The Rust reader uses the 16-byte filename fields, decodes the row-major tile
grid, bounds-checks both tail arrays, and checks that both referenced PDT
resources resolve. The count formula accounts for the entire tail of all 226
supplied MAP files. `FA` updates the 17-byte entity records by the word ID at
offset 6, matching `sub_1BD89`. The low nine bits of each tile word select a
16×16 chip from the base PDT's rectangle; the remaining bits hold collision,
layer, and dynamic state (`sub_1D671`). The Rust renderer draws a 40×25 tile
viewport and keeps the 4bpp palette indices. All 226 supplied MAP files
render within their referenced chip atlases. The 10-byte trigger semantics,
actor overlay, and high tile-bit effects still need to be reconstructed.

## Window objects

`W1` passes seven evaluated values to `sub_1690E`, which forwards the window
bounds and object ID to `sub_1669B`. The latter converts x coordinates from
two eight-pixel units and y coordinates from sixteen-pixel units. The Rust
player stores these rectangles by object ID and places `U2`/`W6` text in the
matching rectangle. Multiple text objects remain visible until `W4` or `WA`
closes the corresponding ID. The two style attributes and full window draw
order still need to be matched to the executable.

`W8` passes its object ID, width, and row-style flag to `sub_1AAF9`. For the
supplied scripts, style zero uses 24-pixel rows and other values use 16-pixel
rows. The desktop player now uses the width operand for both menu drawing and
mouse hit-testing. Positioning controlled by the reference's focus/mouse state
still needs further work.

## PDT34 status

All 350 Sorcer Kingdom `.PDT` files in the supplied directory begin with `0x34`. The next 32 bytes decode as sixteen little-endian 12-bit palette values. This format remains separate from AVG32/RealLive `PDT10`/`PDT11`.

The display path is recovered from `sub_22893`, `sub_226B7`, and `sub_225BE`. `sub_22893` consumes the tag byte before calling the image decoder. The 32-byte palette block is followed by RLE marker bytes at file offsets `0x21` and `0x22`, four little-endian rectangle bounds at `0x23..0x2b`, then four planar streams beginning at `0x2b`. Each plane stores byte-columns and pairs of scanlines; the PC-98 display buffer is 640x400. The Rust decoder uses this layout and decodes all 350 supplied PDT files. Palette words use the PC-98 analog palette order `0xGRB` (green in bits 8..11, red in bits 4..7, blue in bits 0..3): `sub_1F676` writes them to ports AAh/ACh/AEh in that order without reordering, and `COLOR.TBL` banks use the same layout.

## Engine port

`engine::Engine` ports `UK2.EXE` routine by routine. Important data-segment
locations (DS linear base `0x23610`; IDA `word_2XXXX` is DS offset
`0x2XXXX - 0x23610`):

- The top loop (`sub_14A52`) reruns the start MES while the interpreter
  returns status 5 (`U7` load and the J3/U7 restart path). The restart name is at
  `unk_28EDA`.
- The expression accumulator at `DS:38D7` is not cleared between expressions;
  only nested operands zero it.
- Objects: `dword_23B72` holds 70 far pointers into a pool of `0x62`-byte
  records, and `word_2901C` counts them. `sub_1F505` rewrites its index
  argument in place (-1 selects the top object), and callers reuse that
  index.
- Composition (`sub_16455`): two 40x25 cell-owner maps (`DS:59D0`, `0xF0`
  = background). Windows are drawn on page 0, and uncovered cells are copied
  back from the page-1 background.
- Text (`sub_16F89`, `sub_171C6`): `U2` formats into the object's `0x7D0`-byte
  buffer. Single-byte characters map through the zenkaku table at the far
  pointer `DS:F2`, and half-width glyphs come from `kana.pdt`, captured at
  start-up into `DS:5F2A`.
- Maps: a `0x3A`-byte header, 10-byte cell buffers double-buffered through
  `DS:3B52`, and 17-byte entities, including the global entity list loaded
  from the file named by the config `FONT` key.
- Battles: party slots at `DS:391C` (13 x `0x1D`), `FIGHT.TAB` and `LEVEL.TAB`.
- Random numbers use the Borland C runtime LCG.

## Implementation boundary

The engine port covers the interpreter and all two-character service
commands used by the supplied scripts. The EMS effect sequencer (`A*`) stays
inactive, like it does without EMS. The PMD port omits sound effects, the
FM3 extended parts, and the ADPCM part J, which needs a sample bank the game
does not ship. The older format layer (`Uk2Vm`/`Uk2Host`) still surfaces
decoded service commands for `uk2_verify`.
