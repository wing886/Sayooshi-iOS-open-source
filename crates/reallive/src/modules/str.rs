//! String functions (module 1:10).
//!
//! Strings are stored decoded. Lengths and offsets that RealLive measures
//! in Shift-JIS bytes (`strlen`, `strpos`, `strcpy`'s count) are measured
//! in half-width cells here, which is the same number for Japanese text
//! and stays meaningful for other encodings; character counts
//! (`strcharlen`, `strsub`) count characters.

use anyhow::{Result, bail};

use crate::bytecode::Command;
use crate::machine::{Machine, Next, StrTarget};
use crate::nls::{self, cell_width, text_width};

/// The prefix of `text` that fits in `cells` half-width cells.
pub fn truncate_cells(text: &str, cells: usize) -> String {
    let mut used = 0;
    text.chars()
        .take_while(|&c| {
            used += cell_width(c);
            used <= cells
        })
        .collect()
}

/// Cell offset of the character at char index `chars`.
fn cells_before(text: &str, chars: usize) -> usize {
    text.chars().take(chars).map(cell_width).sum()
}

/// Number formatting shared by the `itoa` family: `width` pads with
/// `fill` after the sign (as `%0*d` would for `'0'`).
pub fn itoa(number: i32, width: i32, fill: char) -> String {
    let digits = number.unsigned_abs().to_string();
    let mut out = String::new();
    if number < 0 {
        out.push('-');
    }
    let pad = (width.max(0) as usize).saturating_sub(digits.len() + out.len());
    // std::setw applies to the magnitude only in the original.
    let pad = if width > 0 {
        (width as usize).saturating_sub(digits.len())
    } else {
        pad
    };
    for _ in 0..pad {
        out.push(fill);
    }
    out.push_str(&digits);
    out
}

/// `atoi`: leading whitespace, optional sign, digits; 0 when none.
pub fn atoi(text: &str) -> i32 {
    let text = nls::zen_to_han(text);
    let trimmed = text.trim_start();
    let (negative, digits) = match trimmed.as_bytes().first() {
        Some(b'-') => (true, &trimmed[1..]),
        Some(b'+') => (false, &trimmed[1..]),
        _ => (false, trimmed),
    };
    let value = digits
        .bytes()
        .take_while(u8::is_ascii_digit)
        .fold(0i64, |acc, digit| {
            (acc * 10 + i64::from(digit - b'0')).min(i64::from(u32::MAX))
        });
    let value = if negative { -value } else { value };
    value.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32
}

fn set(machine: &mut Machine, target: StrTarget, value: String) -> Result<()> {
    machine.write_string(target, value)
}

pub fn dispatch(machine: &mut Machine, command: &Command) -> Result<Next> {
    let op = command.op;
    match (op.opcode, op.overload) {
        // strcpy(dest, src[, count])
        (0, overload) => {
            let dest = machine.str_target_param(command, 0)?;
            let mut value = machine.str_param(command, 1)?;
            if overload == 1 {
                let count = machine.int_param(command, 2)?.max(0) as usize;
                value = truncate_cells(&value, count);
            }
            set(machine, dest, value)?;
        }
        // strclear(dest) / strclear(first, last)
        (1, 0) => {
            let dest = machine.str_target_param(command, 0)?;
            set(machine, dest, String::new())?;
        }
        (1, _) => {
            let first = machine.str_target_param(command, 0)?;
            let last = machine.str_target_param(command, 1)?;
            if first.bank != last.bank {
                bail!("reallive: strclear range spans two banks");
            }
            for index in first.index..=last.index {
                set(machine, StrTarget { index, ..first }, String::new())?;
            }
        }
        // strcat(dest, src)
        (2, _) => {
            let dest = machine.str_target_param(command, 0)?;
            let mut value = machine.read_string(dest)?;
            value.push_str(&machine.str_param(command, 1)?);
            set(machine, dest, value)?;
        }
        // strlen(s)
        (3, _) => {
            let value = machine.str_param(command, 0)?;
            machine.store = text_width(&value) as i32;
        }
        // strcmp(a, b)
        (4, _) => {
            let nls = machine.nls();
            let left = nls.encode(&machine.str_param(command, 0)?);
            let right = nls.encode(&machine.str_param(command, 1)?);
            machine.store = match left.cmp(&right) {
                std::cmp::Ordering::Less => -1,
                std::cmp::Ordering::Equal => 0,
                std::cmp::Ordering::Greater => 1,
            };
        }
        // strsub(dest, src, offset[, length]) / strrsub
        (5 | 6, overload) => {
            let dest = machine.str_target_param(command, 0)?;
            let source = machine.str_param(command, 1)?;
            let mut offset = machine.int_param(command, 2)?;
            let length = if overload == 1 {
                Some(machine.int_param(command, 3)?)
            } else {
                None
            };
            let chars: Vec<char> = source.chars().collect();
            if op.opcode == 6 {
                offset = chars.len() as i32 - offset;
            }
            // Out of range: whatever part of the string is left (scripts
            // run past the end, e.g. LB's SEEN9515).
            let tail = &chars[(offset.max(0) as usize).min(chars.len())..];
            let take = length.map_or(tail.len(), |length| {
                (length.max(0) as usize).min(tail.len())
            });
            set(machine, dest, tail[..take].iter().collect())?;
        }
        // strcharlen(s)
        (7, _) => {
            let value = machine.str_param(command, 0)?;
            machine.store = value.chars().count() as i32;
        }
        // strtrunc(dest, chars)
        (8, _) => {
            let dest = machine.str_target_param(command, 0)?;
            let length = machine.int_param(command, 1)?.max(0) as usize;
            let value: String = machine.read_string(dest)?.chars().take(length).collect();
            set(machine, dest, value)?;
        }
        // hantozen / zentohan / Uppercase / Lowercase: (dest) or (src, dest)
        (10..=13, overload) => {
            let (source, dest) = if overload == 0 {
                let dest = machine.str_target_param(command, 0)?;
                (machine.read_string(dest)?, dest)
            } else {
                (
                    machine.str_param(command, 0)?,
                    machine.str_target_param(command, 1)?,
                )
            };
            let value = match op.opcode {
                10 => nls::han_to_zen(&source),
                11 => nls::zen_to_han(&source),
                12 => source.to_ascii_uppercase(),
                _ => source.to_ascii_lowercase(),
            };
            set(machine, dest, value)?;
        }
        // itoa_ws / itoa_s / itoa_w / itoa: (value, dest[, length])
        (14..=17, overload) => {
            let value = machine.int_param(command, 0)?;
            let dest = machine.str_target_param(command, 1)?;
            let length = if overload == 1 {
                machine.int_param(command, 2)?
            } else {
                -1
            };
            let fill = if matches!(op.opcode, 14 | 15) {
                ' '
            } else {
                '0'
            };
            let mut text = itoa(value, length, fill);
            if matches!(op.opcode, 14 | 16) {
                text = nls::han_to_zen(&text);
            }
            set(machine, dest, text)?;
        }
        // atoi(s)
        (18, _) => {
            let value = machine.str_param(command, 0)?;
            machine.store = atoi(&value);
        }
        // digits(n)
        (19, _) => {
            let value = machine.int_param(command, 0)?;
            machine.store = value.unsigned_abs().to_string().len() as i32;
        }
        // digit(n, dest, index): the index-th digit from the right.
        (20, _) => {
            let value = machine.int_param(command, 0)?;
            let dest = machine.int_target_param(command, 1)?;
            let index = machine.int_param(command, 2)?;
            let digits = value.unsigned_abs().to_string().into_bytes();
            let digit = usize::try_from(index)
                .ok()
                .filter(|&index| index >= 1 && index <= digits.len())
                .map_or(0, |index| i32::from(digits[digits.len() - index] - b'0'));
            machine.set_target(dest, digit)?;
            machine.store = digits.len() as i32;
        }
        // strpos / strlpos
        (30 | 31, _) => {
            let haystack = machine.str_param(command, 0)?;
            let needle = machine.str_param(command, 1)?;
            let found = if op.opcode == 30 {
                haystack.find(&needle)
            } else {
                haystack.rfind(&needle)
            };
            machine.store = found.map_or(-1, |byte| {
                cells_before(&haystack, haystack[..byte].chars().count()) as i32
            });
        }
        // STRFIND / STRFIND_REAL(s, first, last): the first string
        // variable (strS) holding `s` (ignoring case, or exactly), written
        // to both outputs; `store` is 1 when found. (Only the signature is
        // known; this reading is a guess.)
        (32 | 33, _) => {
            let needle = machine.str_param(command, 0)?;
            let exact = command.op.opcode == 33;
            let mut found = None;
            for index in 0..crate::memory::BANK_SIZE as i32 {
                let target = StrTarget {
                    bank: crate::expr::bank::STR_S,
                    index,
                };
                let value = machine.read_string(target)?;
                let hit = if exact {
                    value == needle
                } else {
                    value.eq_ignore_ascii_case(&needle)
                };
                if hit {
                    found = Some(index);
                    break;
                }
            }
            for at in 1..command.params.len().min(3) {
                let target = machine.int_target_param(command, at)?;
                machine.set_target(target, found.unwrap_or(-1))?;
            }
            machine.store = i32::from(found.is_some());
        }
        // STRCOPY_BUFSIZE(dest, src, size) / STRSET_BUFSIZE(buf, size): as
        // if through a C buffer of `size` bytes (terminator included).
        (40 | 41, _) => {
            let dest = machine.str_target_param(command, 0)?;
            let (text, size) = if command.op.opcode == 40 {
                (
                    machine.str_param(command, 1)?,
                    machine.int_param(command, 2)?,
                )
            } else {
                (machine.read_string(dest)?, machine.int_param(command, 1)?)
            };
            let mut bytes = 0;
            let fitted: String = text
                .chars()
                .take_while(|c| {
                    bytes += if c.is_ascii() || ('\u{FF61}'..='\u{FF9F}').contains(c) {
                        1
                    } else {
                        2
                    };
                    bytes < size.max(1)
                })
                .collect();
            set(machine, dest, fitted)?;
        }
        // strout(s) / intout(n)
        (100, 0) => {
            let value = machine.str_param(command, 0)?;
            crate::modules::msg::textout(machine, &value)?;
        }
        (100, _) => {
            let value = machine.int_param(command, 0)?;
            crate::modules::msg::textout(machine, &value.to_string())?;
        }
        // strused(s)
        (200, _) => {
            let target = machine.str_target_param(command, 0)?;
            machine.store = i32::from(!machine.read_string(target)?.is_empty());
        }
        _ => return machine.unimplemented(command),
    }
    Ok(Next::Advance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn itoa_matches_the_originals_padding() {
        assert_eq!(itoa(42, -1, '0'), "42");
        assert_eq!(itoa(42, 5, '0'), "00042");
        assert_eq!(itoa(-42, 5, ' '), "-   42");
        assert_eq!(atoi("  -12abc"), -12);
        assert_eq!(atoi("１２"), 12);
        assert_eq!(truncate_cells("あいう", 3), "あ");
    }
}
