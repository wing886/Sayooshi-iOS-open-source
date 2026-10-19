//! Reverse-engineered support for AyPio's UK2 engine.
//!
//! The initial reference implementation is the PC-98 release of Sorcer
//! Kingdom.  UK2 is a separate VM and resource family from AVG32, RealLive,
//! and SiglusEngine, so its bytecode and assets live in their own crate.

pub mod color;
pub mod config;
pub mod disasm;
pub mod dlb;
pub mod engine;
pub mod game;
pub mod map;
pub mod mes;
pub mod music;
pub mod opcode;
pub mod pdt;
pub mod value;
pub mod vm;

pub use color::ColorTable;
pub use config::Uk2Config;
pub use disasm::{Instruction, InstructionArg, InstructionKind, disassemble_reachable};
pub use dlb::{DlbArchive, DlbEntry};
pub use game::Uk2Game;
pub use map::{UK2_MAP_HEADER_SIZE, Uk2MapLayout};
pub use mes::{MES_CODE_OFFSET, MES_MAGIC, MesProgram};
pub use music::{MmdRecord, MmdTrack, MmmTrack, Uk2MusicFile, Uk2MusicKind};
pub use opcode::{SingleOpcode, TwoOpcode};
pub use pdt::{
    PDT34_DATA_OFFSET, PDT34_HEIGHT, PDT34_WIDTH, Pdt34Header, Pdt34Image, decode_pdt34,
};
pub use value::{Condition, Expression, Operand};
pub use vm::{NumericWidth, ResolvedArg, RuntimeValue, ServiceCommand, Uk2Host, Uk2Memory, Uk2Vm};
