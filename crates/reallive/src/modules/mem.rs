//! Block memory operations (module 1:11).

use anyhow::Result;

use crate::bytecode::Command;
use crate::machine::{IntTarget, Machine, Next};

pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let op = command.op;
    match (op.opcode, op.overload) {
        // setarray(origin, values+)
        (0, _) => {
            let origin = machine.int_target_param(command, 0)?;
            let values = machine.int_params_from(command, 1)?;
            for (target, value) in Machine::int_range(origin, values.len())
                .into_iter()
                .zip(values)
            {
                machine.set_target(target, value)?;
            }
        }
        // setrng(first, last[, value])
        (1, _) => {
            let first = machine.int_target_param(command, 0)?;
            let last = machine.int_target_param(command, 1)?;
            let value = machine.int_param_or(command, 2, 0)?;
            for target in Machine::int_span(first, last)? {
                machine.set_target(target, value)?;
            }
        }
        // cpyrng(source, dest, count)
        (2, _) => {
            let source = machine.int_target_param(command, 0)?;
            let dest = machine.int_target_param(command, 1)?;
            let count = machine.int_param(command, 2)?.max(0) as usize;
            let values = Machine::int_range(source, count)
                .into_iter()
                .map(|target| machine.get_target(target))
                .collect::<Result<Vec<_>>>()?;
            for (target, value) in Machine::int_range(dest, count).into_iter().zip(values) {
                machine.set_target(target, value)?;
            }
        }
        // setarray_stepped(origin, step, values+)
        (3, _) => {
            let origin = machine.int_target_param(command, 0)?;
            let step = machine.int_param(command, 1)?;
            let values = machine.int_params_from(command, 2)?;
            for (i, value) in values.into_iter().enumerate() {
                machine.set_target(offset(origin, step * i as i32), value)?;
            }
        }
        // setrng_stepped(origin, step, count[, value])
        (4, _) => {
            let origin = machine.int_target_param(command, 0)?;
            let step = machine.int_param(command, 1)?;
            let count = machine.int_param(command, 2)?;
            let value = machine.int_param_or(command, 3, 0)?;
            for i in 0..count.max(0) {
                machine.set_target(offset(origin, step * i), value)?;
            }
        }
        // cpyvars(dest, offset, sources+)
        (6, _) => {
            let dest = machine.int_target_param(command, 0)?;
            let shift = machine.int_param(command, 1)?;
            let mut values = Vec::new();
            for index in 2..command.params.len() {
                let source = machine.int_target_param(command, index)?;
                values.push(machine.get_target(offset(source, shift))?);
            }
            for (target, value) in Machine::int_range(dest, values.len())
                .into_iter()
                .zip(values)
            {
                machine.set_target(target, value)?;
            }
        }
        // sum(first, last)
        (100, _) => {
            let first = machine.int_target_param(command, 0)?;
            let last = machine.int_target_param(command, 1)?;
            machine.store = sum(machine, first, last)?;
        }
        // sums((first, last)+)
        (101, _) => {
            let mut total = 0i32;
            for index in 0..command.params.len() {
                let pair = machine.complex_param(command, index)?;
                let (Some(first), Some(last)) = (pair.first(), pair.get(1)) else {
                    continue;
                };
                let first = machine.int_target(first)?;
                let last = machine.int_target(last)?;
                total = total.wrapping_add(sum(machine, first, last)?);
            }
            machine.store = total;
        }
        // FLAGSORT_LARGE / FLAGSORT_SMALL(count, keys, values...): sorts the
        // keys (descending / ascending) and reorders the values with them.
        (200 | 201, overload) => {
            let count = machine.int_param(command, 0)?.max(0) as usize;
            let keys_at = machine.int_target_param(command, 1)?;
            let keys = read_ints(machine, keys_at, count)?;
            let order = sorted_order(&keys, op.opcode == 200);
            write_ints(
                machine,
                keys_at,
                &order.iter().map(|&i| keys[i]).collect::<Vec<_>>(),
            )?;
            let strings = matches!(overload, 1 | 3)
                || (overload == 0 && command.params.get(2).is_some_and(|p| p.value.is_string()));
            let (source, dest) = if overload >= 2 { (2, 3) } else { (2, 2) };
            if strings {
                let from = machine.str_target_param(command, source)?;
                let to = machine.str_target_param(command, dest)?;
                let values = (0..count)
                    .map(|i| machine.read_string(offset_str(from, i)))
                    .collect::<Result<Vec<_>>>()?;
                for (slot, &i) in order.iter().enumerate() {
                    machine.write_string(offset_str(to, slot), values[i].clone())?;
                }
            } else {
                let from = machine.int_target_param(command, source)?;
                let to = machine.int_target_param(command, dest)?;
                let values = read_ints(machine, from, count)?;
                write_ints(
                    machine,
                    to,
                    &order.iter().map(|&i| values[i]).collect::<Vec<_>>(),
                )?;
            }
        }
        // FLAGINDEXSORT_LARGE / _SMALL(count, keys[, dest[, first index]]):
        // the key indices in sorted order.
        (210 | 211, _) => {
            let count = machine.int_param(command, 0)?.max(0) as usize;
            let keys_at = machine.int_target_param(command, 1)?;
            let keys = read_ints(machine, keys_at, count)?;
            let order = sorted_order(&keys, op.opcode == 210);
            let top = machine.int_param_or(command, 3, 0)?;
            let dest = if command.params.len() > 2 {
                machine.int_target_param(command, 2)?
            } else {
                keys_at
            };
            let indices: Vec<i32> = order.iter().map(|&i| i as i32 + top).collect();
            write_ints(machine, dest, &indices)?;
        }
        // FLAGCOPY_INDEX(source, source offset, dest, dest offset, count)
        (5, _) => {
            let source = machine.int_target_param(command, 0)?;
            let source_shift = machine.int_param(command, 1)?;
            let dest = machine.int_target_param(command, 2)?;
            let dest_shift = machine.int_param(command, 3)?;
            let count = machine.int_param_or(command, 4, 1)?.max(0) as usize;
            let values = read_ints(machine, offset(source, source_shift), count)?;
            write_ints(machine, offset(dest, dest_shift), &values)?;
        }
        (300..=307, _) => database(machine, command)?,
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

fn offset(target: IntTarget, by: i32) -> IntTarget {
    match target {
        IntTarget::Mem(mut reference) => {
            reference.index += by;
            IntTarget::Mem(reference)
        }
        IntTarget::Store => IntTarget::Store,
    }
}

fn sum(machine: &Machine, first: IntTarget, last: IntTarget) -> Result<i32> {
    Machine::int_span(first, last)?
        .into_iter()
        .try_fold(0i32, |total, target| {
            Ok(total.wrapping_add(machine.get_target(target)?))
        })
}

/// Indices of `keys` in sorted order (stable).
fn sorted_order(keys: &[i32], descending: bool) -> Vec<usize> {
    let mut order: Vec<usize> = (0..keys.len()).collect();
    if descending {
        order.sort_by_key(|&i| std::cmp::Reverse(keys[i]));
    } else {
        order.sort_by_key(|&i| keys[i]);
    }
    order
}

fn read_ints(machine: &mut Machine, first: IntTarget, count: usize) -> Result<Vec<i32>> {
    Machine::int_range(first, count)
        .into_iter()
        .map(|target| machine.get_target(target))
        .collect()
}

fn write_ints(machine: &mut Machine, first: IntTarget, values: &[i32]) -> Result<()> {
    for (target, &value) in Machine::int_range(first, values.len())
        .into_iter()
        .zip(values)
    {
        machine.set_target(target, value)?;
    }
    Ok(())
}

fn offset_str(target: crate::machine::StrTarget, offset: usize) -> crate::machine::StrTarget {
    crate::machine::StrTarget {
        index: target.index + offset as i32,
        ..target
    }
}

/// Database `number` (`#DATABASE.nnn = "name"`, the table in
/// `dat/name.dbs`), loaded once.
fn load_database(
    machine: &mut Machine,
    number: i32,
) -> Option<std::rc::Rc<siglus_assets::dbs::DbsDatabase>> {
    if let Some(loaded) = machine.sys.databases.get(&number) {
        return loaded.clone();
    }
    let entry = machine
        .gameexe
        .get(&format!("DATABASE.{number:03}"))
        .or_else(|| machine.gameexe.get(&format!("DATABASE.{number}")));
    let name = entry.and_then(|e| e.str(0)).map(str::to_owned);
    let loaded = name.and_then(|name| {
        let resources = &machine.sys.resources;
        let path = resources
            .root_file(&format!("dat/{name}.dbs"))
            .or_else(|| resources.root_file(&format!("{name}.dbs")))?;
        match siglus_assets::dbs::DbsDatabase::load(&path) {
            Ok(db) => Some(std::rc::Rc::new(db)),
            Err(error) => {
                machine.report(format!("database {name}: {error:#}"));
                None
            }
        }
    });
    machine.sys.databases.insert(number, loaded.clone());
    loaded
}

/// `DATABASEGET_NUM/_STR(db, item, column, var)`, `DATABASEGET(db, item,
/// (column, var[, count])...)`, `DATABASECHECK_ITEM/_COLUMN`,
/// `DATABASEFIND_NUM/_STR/_STR_REAL(db, column, value)`.
fn database(machine: &mut Machine, command: &Command) -> Result<()> {
    let opcode = command.op.opcode;
    let number = machine.int_param(command, 0)?;
    let db = load_database(machine, number);
    match opcode {
        300 | 301 => {
            let item = machine.int_param(command, 1)?;
            let column = machine.int_param(command, 2)?;
            if opcode == 300 {
                let value = match &db {
                    Some(db) => db.get_data_int(item, column)?.unwrap_or(0),
                    None => 0,
                };
                let target = machine.int_target_param(command, 3)?;
                machine.set_target(target, value)?;
            } else {
                let value = match &db {
                    Some(db) => db.get_data_str(item, column)?.unwrap_or_default(),
                    None => String::new(),
                };
                let target = machine.str_target_param(command, 3)?;
                machine.write_string(target, value)?;
            }
        }
        302 => {
            let item = machine.int_param(command, 1)?;
            for index in 2..command.params.len() {
                let pieces = machine.complex_param(command, index)?;
                let [column, target, rest @ ..] = &pieces[..] else {
                    continue;
                };
                let column = machine.eval_int(column)?;
                let count = match rest.first() {
                    Some(count) => machine.eval_int(count)?.max(0),
                    None => 1,
                };
                if target.is_string() {
                    let first = machine.str_target(target)?;
                    for i in 0..count {
                        let value = match &db {
                            Some(db) => db.get_data_str(item, column + i)?.unwrap_or_default(),
                            None => String::new(),
                        };
                        machine.write_string(offset_str(first, i as usize), value)?;
                    }
                } else {
                    let first = machine.int_target(target)?;
                    for i in 0..count {
                        let value = match &db {
                            Some(db) => db.get_data_int(item, column + i)?.unwrap_or(0),
                            None => 0,
                        };
                        machine.set_target(offset(first, i), value)?;
                    }
                }
            }
        }
        303 | 304 => {
            let value = machine.int_param(command, 1)?;
            machine.store = match &db {
                Some(db) if opcode == 303 => db.check_item_no(value),
                Some(db) => db.check_column_no(value),
                None => 0,
            };
        }
        _ => {
            let column = machine.int_param(command, 1)?;
            machine.store = match (&db, opcode) {
                (None, _) => -1,
                (Some(db), 305) => {
                    let value = machine.int_param(command, 2)?;
                    db.find_num(column, value)?
                }
                (Some(db), 306) => db.find_str(column, &machine.str_param(command, 2)?)?,
                (Some(db), _) => db.find_str_real(column, &machine.str_param(command, 2)?)?,
            };
        }
    }
    Ok(())
}
