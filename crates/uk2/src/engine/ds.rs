//! Data-segment variable offsets of the reference `UK2.EXE`.
//!
//! Each constant is written as `l(0xLINEAR)` with the linear address used by
//! the IDA listing label (`word_2901C`, `byte_28FE9`, ...), so the port can be
//! checked line-by-line against the disassembly.

pub const fn l(linear: u32) -> u16 {
    (linear - 0x23610) as u16
}

// ---- interpreter -------------------------------------------------------
/// word_271A6: current MES program counter.
pub const MES_PC: u16 = l(0x271A6);
/// word_271A8: current MES length.
pub const MES_LEN: u16 = l(0x271A8);
/// word_271AA: MES buffer stack depth (bytes used by outer programs).
pub const MES_STACK: u16 = l(0x271AA);
/// word_2716A: last operand numeric width (1 = word, 0 = byte).
pub const OPERAND_WORD: u16 = l(0x2716A);
/// word_2716C: last operand kind (0 = number, 1 = string, 2 = large string).
pub const OPERAND_KIND: u16 = l(0x2716C);
/// word_2716E/word_27170: far pointer to the class-2 local table.
pub const LOCAL_TABLE: u16 = l(0x2716E);
/// dword_29076: current MES record (T0 slots at +0, count at +1A4).
pub const MES_RECORD: u16 = l(0x29076);
/// word_2907E: condition flag of the last L-family test.
pub const COND_FLAG: u16 = l(0x2907E);
/// word_26714: class-0 immediate scratch word.
pub const IMM_SCRATCH: u16 = l(0x26714);
/// word_26EE7/word_26EE9: 32-bit expression accumulator (DS:38D7).
pub const ACC_LO: u16 = l(0x26EE7);
pub const ACC_HI: u16 = l(0x26EE9);
/// DS:3106 inline string assembly buffer.
pub const INLINE_BUF: u16 = 0x3106;
pub const SYSTEM_WORDS: u16 = 0x3B62;
pub const LARGE_STRINGS: u16 = 0x3B9C;
pub const FIXED_STRINGS: u16 = 0x5312;
pub const INDEXED_8: u16 = 0x56B4;
pub const INDEXED_5: u16 = 0x57B4;
pub const GLOBAL_SLOTS: u16 = 0x5A12;
pub const INDIRECT_TABLE: u16 = 0x2B8;
/// unk_28ECC: current MES name; unk_28EDA: J3/restart MES name.
pub const CUR_MES_NAME: u16 = l(0x28ECC);
pub const NEXT_MES_NAME: u16 = l(0x28EDA);
/// word_28FCA: two-character opcodes enabled.
pub const OPCODES_ENABLED: u16 = l(0x28FCA);
/// word_25DE0: CTRL+GRPH+SHIFT abort request.
pub const ABORT: u16 = l(0x25DE0);
/// word_23B64: skip interpreter pre-instruction service.
pub const SKIP_SERVICE: u16 = l(0x23B64);

// ---- window objects -----------------------------------------------------
/// dword_23B72: far pointer to the object pointer array (70 entries).
pub const OBJ_TABLE: u16 = l(0x23B72);
/// word_2901C: live object count.
pub const OBJ_COUNT: u16 = l(0x2901C);
/// word_23B6E/70: MAKE_WIN.TAB template records, count word_28FEA.
pub const TEMPLATES: u16 = l(0x23B6E);
pub const TEMPLATE_COUNT: u16 = l(0x28FEA);
/// word_28FF4: id of the top object after the last composition.
pub const TOP_OBJECT: u16 = l(0x28FF4);
/// byte_28FE9 / byte_28FE8: object the interpreter waits on (W7/WF/WI).
pub const WAIT_OBJECT: u16 = l(0x28FE9);
pub const WAIT_FLAG: u16 = l(0x28FE8);
/// word_28FEE: wait mode (0 page wait, 1 WI, 2 released).
pub const WAIT_MODE: u16 = l(0x28FEE);
/// word_2713E: text finished flag.
pub const TEXT_DONE: u16 = l(0x2713E);
/// word_29006: force redraw of unchanged cells.
pub const FORCE_REDRAW: u16 = l(0x29006);
/// word_2900E: which of the two cell-owner maps is current.
pub const CELL_MAP_SEL: u16 = l(0x2900E);
/// DS:59D0: two far pointers to 40x25 cell-owner maps.
pub const CELL_MAPS: u16 = 0x59D0;
pub const W_29008: u16 = l(0x29008);
pub const FRAME_STYLE: u16 = l(0x29010);
pub const W_29004: u16 = l(0x29004);
pub const W_29002: u16 = l(0x29002);
pub const W_29000: u16 = l(0x29000);
pub const W_29012: u16 = l(0x29012);
pub const W_29014: u16 = l(0x29014);
pub const W_29016: u16 = l(0x29016);
pub const W_29018: u16 = l(0x29018);
pub const W_2901A: u16 = l(0x2901A);
pub const W_28FC4: u16 = l(0x28FC4);
pub const W_28FC6: u16 = l(0x28FC6);
pub const W_28FCC: u16 = l(0x28FCC);
pub const W_28FEC: u16 = l(0x28FEC);
pub const W_28FF0: u16 = l(0x28FF0);
pub const W_28FF2: u16 = l(0x28FF2);
pub const W_28FF8: u16 = l(0x28FF8);
pub const W_28FFA: u16 = l(0x28FFA);
pub const W_28FFC: u16 = l(0x28FFC);
pub const W_28FFE: u16 = l(0x28FFE);
pub const W_2900A: u16 = l(0x2900A);
pub const W_2900C: u16 = l(0x2900C);
pub const W_27140: u16 = l(0x27140);
pub const W_27144: u16 = l(0x27144);
pub const W_27152: u16 = l(0x27152);
/// word_23A5E: text speed (0 = normal, 1000 = instant).
pub const TEXT_SPEED: u16 = l(0x23A5E);
/// Window frame chip image (win.pdt1) and frame parts (win_disp.pdt1).
pub const WIN_CHIPS: u16 = l(0x28FDC);
pub const WIN_DISP_CHIPS: u16 = l(0x28FD4);
/// word_2710A: glyph drawing style.
pub const GLYPH_STYLE: u16 = l(0x2710A);
/// Text colours: word_28FBE shadow, word_28FC0 outline, word_28FC2 body.
pub const TEXT_SHADOW: u16 = l(0x28FBE);
pub const TEXT_OUTLINE: u16 = l(0x28FC0);
pub const TEXT_COLOUR: u16 = l(0x28FC2);
/// word_25D1E: glyph raster op (1..4).
pub const GLYPH_OP: u16 = l(0x25D1E);
/// word_25D20: last glyph was single-byte.
pub const GLYPH_HALF: u16 = l(0x25D20);
/// far pointer to the half-width font captured from kana.pdt1.
pub const ANK_FONT: u16 = l(0x25D22);
pub const ANK_FONT_DATA: u16 = 0x5F2A;
/// dword at DS:F2 -> hankaku to zenkaku table.
pub const ZENKAKU_TABLE: u16 = l(0x23702);
/// DS:2716 colour remap used by glyph drawing.
pub const GLYPH_COLOUR_MAP: u16 = 0x2716;

// ---- graphics ------------------------------------------------------------
/// word_29085 display page, word_29083 access page.
pub const DISPLAY_PAGE: u16 = l(0x29085);
pub const ACCESS_PAGE: u16 = l(0x29083);
/// DS:67C current palette, DS:69C + n*32 COLOR.TBL banks.
pub const PALETTE: u16 = 0x67C;
pub const PALETTE_BANKS: u16 = 0x69C;
pub const PALETTE_BANK_COUNT: u16 = l(0x23DEC);
pub const PALETTE_LOCK: u16 = l(0x23DEE);
/// word_25DC4: PDT loads copy their palette into bank 1.
pub const PDT_PALETTE: u16 = l(0x25DC4);
/// word_29F54/52/50/4E: last PDT rectangle (left, right, top, bottom).
pub const PDT_LEFT: u16 = l(0x29F54);
pub const PDT_RIGHT: u16 = l(0x29F52);
pub const PDT_TOP: u16 = l(0x29F50);
pub const PDT_BOTTOM: u16 = l(0x29F4E);
/// unk_23B78: current background PDT name.
pub const BG_NAME: u16 = l(0x23B78);
/// word_25C54: EMS in use.
pub const EMS_ACTIVE: u16 = l(0x25C54);
pub const EMS_WANTED: u16 = l(0x25C56);

// ---- input ---------------------------------------------------------------
pub const MOUSE_X: u16 = l(0x25D00);
pub const MOUSE_Y: u16 = l(0x25D02);
pub const MOUSE_MIN_X: u16 = l(0x25D04);
pub const MOUSE_MAX_X: u16 = l(0x25D06);
pub const MOUSE_MIN_Y: u16 = l(0x25D08);
pub const MOUSE_MAX_Y: u16 = l(0x25D0A);
pub const MOUSE_DIV_X: u16 = l(0x25D0C);
pub const MOUSE_DIV_Y: u16 = l(0x25D0E);
pub const MOUSE_MUL_X: u16 = l(0x25D10);
pub const MOUSE_MUL_Y: u16 = l(0x25D12);
pub const MOUSE_PREV_RIGHT: u16 = l(0x25D1A);
pub const MOUSE_PREV_LEFT: u16 = l(0x25D1C);
/// word_294F7 left held, word_29505 right held.
pub const LEFT_HELD: u16 = l(0x294F7);
pub const RIGHT_HELD: u16 = l(0x29505);
/// Press/release event counters.
pub const LEFT_PRESSES: u16 = l(0x294F9);
pub const LEFT_RELEASES: u16 = l(0x294FB);
pub const RIGHT_PRESSES: u16 = l(0x29507);
pub const RIGHT_RELEASES: u16 = l(0x29509);
pub const LEFT_PRESS_X: u16 = l(0x294FD);
pub const LEFT_PRESS_Y: u16 = l(0x294FF);
pub const LEFT_RELEASE_X: u16 = l(0x29501);
pub const LEFT_RELEASE_Y: u16 = l(0x29503);
pub const RIGHT_PRESS_X: u16 = l(0x2950B);
pub const RIGHT_PRESS_Y: u16 = l(0x2950D);
pub const RIGHT_RELEASE_X: u16 = l(0x2950F);
pub const RIGHT_RELEASE_Y: u16 = l(0x29511);
/// byte_294F0: event counter lifetime in vsyncs.
pub const EVENT_LIFETIME: u16 = l(0x294F0);
pub const PRESS_TIMER: u16 = l(0x29256);
pub const RELEASE_TIMER: u16 = l(0x29254);
/// word_294F3: software cursor shown; word_25CFE: cursor mode.
pub const CURSOR_SHOWN: u16 = l(0x294F3);
pub const CURSOR_SHOWN2: u16 = l(0x294F1);
pub const CURSOR_MODE: u16 = l(0x25CFE);
pub const CURSOR_ENABLED: u16 = l(0x25CFA);
pub const CURSOR_SHAPE: u16 = l(0x25CFC);
pub const CURSOR_FRAME: u16 = l(0x25CF8);
pub const CURSOR_FRAMES: u16 = l(0x25CF6);
pub const CURSOR_ANIM_RATE: u16 = l(0x25CF4);
pub const CURSOR_ANIM_COUNT: u16 = l(0x25CEE);
pub const CURSOR_DATA: u16 = 0x83A;
pub const CURSOR_SIZE: u16 = 0x501;
/// DS:6960: key-down table indexed by PC-98 scan code.
pub const KEY_TABLE: u16 = 0x6960;
/// DS:265A: scan code to ASCII table.
pub const KEY_ASCII: u16 = 0x265A;
/// word_25DE4: cursor-key/Return/Space/ESC input enabled.
pub const KEYS_ENABLED: u16 = l(0x25DE4);

// ---- timers --------------------------------------------------------------
pub const TICKS: u16 = l(0x2925A);
pub const TIMER_A: u16 = l(0x29258);
pub const FRAME_TIMER: u16 = l(0x29252);

// ---- map ----------------------------------------------------------------
pub const ENTITIES: u16 = l(0x23B60);
pub const ENTITY_COUNT: u16 = l(0x23B5E);
pub const PLAYER: u16 = l(0x2715E);
pub const LAST_MAP: u16 = l(0x23B8A);
pub const INTERACT_ID: u16 = l(0x2713C);
pub const MAP_EVENT: u16 = l(0x2714E);
pub const FRAME_COUNTER: u16 = l(0x2713A);
pub const FRAME_DELAY: u16 = l(0x27138);
pub const MAP_ACTIVE: u16 = l(0x27148);
pub const MAP_CELL_SEL: u16 = l(0x27154);
pub const MAP_CELLS: u16 = 0x3B52;
pub const CHIP_BASE: u16 = l(0x27110);
pub const CHIP_OVERLAY: u16 = l(0x27114);
pub const SHARED_TILES: u16 = l(0x27118);

// ---- files ----------------------------------------------------------------
pub const SAVE_NAMES: u16 = 0x590C;
pub const SAVE_COMMENT: u16 = l(0x28EF4);
pub const SAVE_DATE: u16 = l(0x28EE8);
pub const ENTITY_FILE: u16 = l(0x2711C);
