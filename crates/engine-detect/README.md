# engine-detect

Identifies which engine a game directory uses: SiglusEngine, RealLive,
AVG32 or UK2. The decision uses on-disk formats only, never publisher or
title names.

| Engine | Evidence |
|--------|----------|
| SiglusEngine | `Scene.pck` or `Gameexe.dat` |
| UK2 | `UK2.CFG`, or `.MES` files starting with `<< UK2 TEXT Ver1.00 >>` |
| AVG32 | `Gameexe.ini` plus a `PACL` `SEEN.TXT`, or loose `SEEN###.TXT` scenes (`TPC32`) |
| RealLive | `Gameexe.ini` plus a 10000-entry `SEEN.TXT` whose scenarios have the `0x1d0` header (`0x1cc` for AVG2000), or loose `SEEN####.TXT` scenes |

RealLive's archive is found through `#FOLDNAME.TXT` in `Gameexe.ini`, then
in the game root and `DAT/`.

```rust
let layout = engine_detect::detect_game_root("path/to/game")?;
println!("{} ({:?})", layout.kind, layout.evidence);
```

The `engine_detect <game-directory>...` binary prints the result and the
evidence for each directory.
