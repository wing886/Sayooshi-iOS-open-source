//! The CG table (`#CGTABLE_FILENAME`, usually `MODE.CGM`): which images
//! count as event CGs and which `intZ` flag records that each was seen.

use anyhow::Result;
use siglus_assets::cgm::CgTableData;

use crate::bytecode::Command;
use crate::machine::Machine;
use crate::memory::{BANK_Z, IntRef};

#[derive(Debug, Default)]
pub struct CgTable {
    data: Option<CgTableData>,
}

impl CgTable {
    pub fn load(bytes: &[u8]) -> Result<Self> {
        Ok(Self {
            data: Some(CgTableData::from_bytes(bytes)?),
        })
    }

    pub fn total(&self) -> i32 {
        self.data
            .as_ref()
            .map_or(0, |data| data.entries.len() as i32)
    }

    /// The flag index of `name` (case-insensitive, extension ignored).
    pub fn flag(&self, name: &str) -> Option<i32> {
        let data = self.data.as_ref()?;
        let wanted = strip_extension(name);
        data.entries
            .iter()
            .find(|entry| strip_extension(&entry.name).eq_ignore_ascii_case(wanted))
            .map(|entry| entry.flag_no)
    }

    /// Every entry, in list order.
    pub fn entries(&self) -> Vec<CgEntry> {
        self.data
            .as_ref()
            .map(|data| {
                data.entries
                    .iter()
                    .map(|e| CgEntry {
                        name: e.name.clone(),
                        flag: e.flag_no,
                        list: e.list_no,
                        code: std::array::from_fn(|i| e.code.get(i).copied().unwrap_or(0)),
                        group: std::array::from_fn(|i| e.group.get(i).copied().unwrap_or(0)),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn flags(&self) -> Vec<i32> {
        self.data
            .as_ref()
            .map(|data| data.entries.iter().map(|entry| entry.flag_no).collect())
            .unwrap_or_default()
    }
}

/// One CG of the table.
#[derive(Debug, Clone, PartialEq)]
pub struct CgEntry {
    pub name: String,
    pub flag: i32,
    pub list: i32,
    pub code: [i32; 5],
    pub group: [i32; 5],
}

fn strip_extension(name: &str) -> &str {
    let name = name.trim();
    match name.rfind('.') {
        Some(dot) if name.len() - dot <= 4 => &name[..dot],
        _ => name,
    }
}

fn flag_value(machine: &Machine, flag: i32) -> i32 {
    machine
        .read_int(IntRef {
            bank: BANK_Z,
            width: 0,
            index: flag,
        })
        .unwrap_or(0)
}

/// Marks `name` as viewed if it is a CG.
pub fn mark_viewed(machine: &mut Machine, name: &str) {
    if !machine.sys.cg_table_enabled {
        return;
    }
    if let Some(flag) = machine.sys.cg_table.flag(name) {
        let reference = IntRef {
            bank: BANK_Z,
            width: 0,
            index: flag,
        };
        if let Err(error) = machine.write_int(reference, 1) {
            machine.report(format!("CG flag for {name}: {error:#}"));
        }
    }
}

/// `cgGetTotal`, `cgGetViewed`, `cgGetViewedPcnt`, `cgGetFlag`, `cgStatus`.
pub fn sys_query(machine: &mut Machine, command: &Command) -> Result<()> {
    let table = &machine.sys.cg_table;
    let viewed = table
        .flags()
        .into_iter()
        .filter(|&flag| flag_value(machine, flag) != 0)
        .count() as i32;
    let total = table.total();
    machine.store = match command.op.opcode {
        1500 => total,
        1501 => viewed,
        1502 => {
            if total == 0 {
                0
            } else {
                (f64::from(viewed) / f64::from(total) * 100.0) as i32
            }
        }
        opcode => {
            let name = machine.str_param(command, 0)?;
            match machine.sys.cg_table.flag(&name) {
                None => -1,
                Some(flag) if opcode == 1503 => flag,
                Some(flag) => i32::from(flag_value(machine, flag) != 0),
            }
        }
    };
    Ok(())
}
