//! C integer semantics the translation relies on: division and remainder
//! truncate towards zero and never trap (a zero divisor yields 0, where the
//! original would have crashed).

#[inline]
pub(crate) fn cdiv_i8(a: i8, b: i8) -> i8 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_i8(a: i8, b: i8) -> i8 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub(crate) fn cdiv_u8(a: u8, b: u8) -> u8 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_u8(a: u8, b: u8) -> u8 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub(crate) fn cdiv_i16(a: i16, b: i16) -> i16 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_i16(a: i16, b: i16) -> i16 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub(crate) fn cdiv_u16(a: u16, b: u16) -> u16 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_u16(a: u16, b: u16) -> u16 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub(crate) fn cdiv_i32(a: i32, b: i32) -> i32 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_i32(a: i32, b: i32) -> i32 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub(crate) fn cdiv_u32(a: u32, b: u32) -> u32 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_u32(a: u32, b: u32) -> u32 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub(crate) fn cdiv_i64(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_i64(a: i64, b: i64) -> i64 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}

#[inline]
pub(crate) fn cdiv_u64(a: u64, b: u64) -> u64 {
    if b == 0 { 0 } else { a.wrapping_div(b) }
}

#[inline]
pub(crate) fn crem_u64(a: u64, b: u64) -> u64 {
    if b == 0 { 0 } else { a.wrapping_rem(b) }
}
