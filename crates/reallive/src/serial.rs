//! A small binary serialization format for saves.
//!
//! Values are little-endian; strings and byte blocks are length-prefixed.
//! Subsystems write named, length-prefixed sections so that readers can
//! skip sections they do not know and saves stay loadable when state is
//! added later.

use anyhow::{Result, bail};

#[derive(Debug, Default, Clone)]
pub struct Writer {
    pub bytes: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn u8(&mut self, value: u8) {
        self.bytes.push(value);
    }

    pub fn bool(&mut self, value: bool) {
        self.u8(u8::from(value));
    }

    pub fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn i32(&mut self, value: i32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn f32(&mut self, value: f32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub fn len(&mut self, value: usize) {
        self.u32(value as u32);
    }

    pub fn bytes(&mut self, value: &[u8]) {
        self.len(value.len());
        self.bytes.extend_from_slice(value);
    }

    pub fn str(&mut self, value: &str) {
        self.bytes(value.as_bytes());
    }

    pub fn i32s(&mut self, values: &[i32]) {
        self.len(values.len());
        for value in values {
            self.i32(*value);
        }
    }

    pub fn strs(&mut self, values: &[String]) {
        self.len(values.len());
        for value in values {
            self.str(value);
        }
    }

    /// A named section written by `body`.
    pub fn section(&mut self, name: &str, body: impl FnOnce(&mut Writer)) {
        let mut inner = Writer::new();
        body(&mut inner);
        self.str(name);
        self.bytes(&inner.bytes);
    }
}

#[derive(Debug, Clone)]
pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    fn take(&mut self, count: usize) -> Result<&'a [u8]> {
        if self.pos + count > self.data.len() {
            bail!("reallive: save data is truncated");
        }
        let slice = &self.data[self.pos..self.pos + count];
        self.pos += count;
        Ok(slice)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    pub fn bool(&mut self) -> Result<bool> {
        Ok(self.u8()? != 0)
    }

    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().expect("4")))
    }

    pub fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().expect("4")))
    }

    pub fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.take(8)?.try_into().expect("8")))
    }

    pub fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.take(4)?.try_into().expect("4")))
    }

    pub fn len(&mut self) -> Result<usize> {
        let value = self.u32()? as usize;
        if value
            > self
                .data
                .len()
                .saturating_sub(self.pos)
                .saturating_mul(8)
                .max(1 << 20)
        {
            bail!("reallive: save data has an implausible length {value}");
        }
        Ok(value)
    }

    pub fn bytes(&mut self) -> Result<&'a [u8]> {
        let count = self.len()?;
        self.take(count)
    }

    pub fn str(&mut self) -> Result<String> {
        Ok(String::from_utf8_lossy(self.bytes()?).into_owned())
    }

    pub fn i32s(&mut self) -> Result<Vec<i32>> {
        let count = self.len()?;
        (0..count).map(|_| self.i32()).collect()
    }

    pub fn strs(&mut self) -> Result<Vec<String>> {
        let count = self.len()?;
        (0..count).map(|_| self.str()).collect()
    }

    /// Reads every section as `(name, body)`.
    pub fn sections(&mut self) -> Result<Vec<(String, &'a [u8])>> {
        let mut out = Vec::new();
        while !self.is_empty() {
            let name = self.str()?;
            let body = self.bytes()?;
            out.push((name, body));
        }
        Ok(out)
    }
}

/// Copies `values` into `slots` (extra values are ignored, missing ones
/// keep their current value).
pub fn fill<T: Clone>(slots: &mut [T], values: &[T]) {
    for (slot, value) in slots.iter_mut().zip(values) {
        *slot = value.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sections_round_trip_and_skip() {
        let mut writer = Writer::new();
        writer.section("a", |w| {
            w.i32(-5);
            w.str("あ");
        });
        writer.section("unknown", |w| w.u64(9));
        let mut reader = Reader::new(&writer.bytes);
        let sections = reader.sections().unwrap();
        assert_eq!(sections.len(), 2);
        let mut a = Reader::new(sections[0].1);
        assert_eq!(a.i32().unwrap(), -5);
        assert_eq!(a.str().unwrap(), "あ");
    }
}
