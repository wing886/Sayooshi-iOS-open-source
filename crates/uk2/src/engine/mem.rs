//! Segmented memory model used by the UK2 engine port.
//!
//! `UK2.EXE` keeps nearly all engine state in its data segment and passes
//! far pointers around freely (the MES operand decoder even returns raw far
//! pointers into DS).  The port therefore keeps the data segment as a real
//! 64 KiB byte array initialised from the game's own `UK2.EXE`, and models
//! every heap allocation (`farmalloc`/`farrealloc`) as its own segment.  A far
//! pointer is `seg << 16 | off`, stored in memory exactly like the original
//! (offset word first, then segment word).

use anyhow::{Context, Result, bail};

pub type FarPtr = u32;

/// Segment value of the original data segment (`seg dseg`).
pub const DS_SEG: u16 = 0x1361;
/// Linear address of DS:0 inside the loaded executable image.
const DS_IMAGE_OFFSET: usize = 0x13610;
const HEAP_SEG_BASE: u16 = 0x4000;

#[inline]
pub fn far(seg: u16, off: u16) -> FarPtr {
    (u32::from(seg) << 16) | u32::from(off)
}

#[inline]
pub fn seg_of(ptr: FarPtr) -> u16 {
    (ptr >> 16) as u16
}

#[inline]
pub fn off_of(ptr: FarPtr) -> u16 {
    ptr as u16
}

#[inline]
pub fn ds_ptr(off: u16) -> FarPtr {
    far(DS_SEG, off)
}

#[inline]
pub fn ptr_add(ptr: FarPtr, delta: i32) -> FarPtr {
    far(seg_of(ptr), (i32::from(off_of(ptr)) + delta) as u16)
}

pub struct Mem {
    pub ds: Box<[u8; 0x10000]>,
    heap: Vec<Option<Vec<u8>>>,
    free_slots: Vec<usize>,
}

impl Mem {
    /// Builds DS from the `UK2.EXE` MZ image. The initialised part of the
    /// segment is copied; the remainder is the zeroed BSS.
    pub fn from_exe(exe: &[u8]) -> Result<Self> {
        if exe.len() < 0x1c || &exe[..2] != b"MZ" {
            bail!("uk2: UK2.EXE is not an MZ executable");
        }
        let header_paragraphs = u16::from_le_bytes([exe[8], exe[9]]) as usize;
        let image = exe
            .get(header_paragraphs * 16..)
            .context("uk2: UK2.EXE header exceeds file")?;
        let data = image
            .get(DS_IMAGE_OFFSET..)
            .context("uk2: UK2.EXE image has no data segment")?;
        const SIGNATURE: &[u8] = b"uk2.exe SOSAR SYSTEM";
        if data.get(0x57e..0x57e + SIGNATURE.len()) != Some(SIGNATURE) {
            bail!("uk2: unsupported UK2.EXE build (data segment signature mismatch)");
        }
        let mut ds = Box::new([0u8; 0x10000]);
        let n = data.len().min(0x10000);
        ds[..n].copy_from_slice(&data[..n]);
        Ok(Self {
            ds,
            heap: Vec::new(),
            free_slots: Vec::new(),
        })
    }

    pub fn blank() -> Self {
        Self {
            ds: Box::new([0u8; 0x10000]),
            heap: Vec::new(),
            free_slots: Vec::new(),
        }
    }

    fn block(&self, seg: u16) -> Option<&Vec<u8>> {
        let index = seg.checked_sub(HEAP_SEG_BASE)? as usize;
        self.heap.get(index)?.as_ref()
    }

    fn block_mut(&mut self, seg: u16) -> Option<&mut Vec<u8>> {
        let index = seg.checked_sub(HEAP_SEG_BASE)? as usize;
        self.heap.get_mut(index)?.as_mut()
    }

    #[inline]
    pub fn rb(&self, ptr: FarPtr) -> u8 {
        let (seg, off) = (seg_of(ptr), off_of(ptr) as usize);
        if seg == DS_SEG {
            return self.ds[off];
        }
        self.block(seg)
            .and_then(|block| block.get(off))
            .copied()
            .unwrap_or(0)
    }

    #[inline]
    pub fn wb(&mut self, ptr: FarPtr, value: u8) {
        let (seg, off) = (seg_of(ptr), off_of(ptr) as usize);
        if seg == DS_SEG {
            self.ds[off] = value;
            return;
        }
        if let Some(slot) = self.block_mut(seg).and_then(|block| block.get_mut(off)) {
            *slot = value;
        }
    }

    #[inline]
    pub fn rw(&self, ptr: FarPtr) -> u16 {
        u16::from_le_bytes([self.rb(ptr), self.rb(ptr_add(ptr, 1))])
    }

    #[inline]
    pub fn ww(&mut self, ptr: FarPtr, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.wb(ptr, lo);
        self.wb(ptr_add(ptr, 1), hi);
    }

    #[inline]
    pub fn rsw(&self, ptr: FarPtr) -> i16 {
        self.rw(ptr) as i16
    }

    /// Reads a far pointer stored as offset word then segment word.
    #[inline]
    pub fn rd(&self, ptr: FarPtr) -> FarPtr {
        far(self.rw(ptr_add(ptr, 2)), self.rw(ptr))
    }

    #[inline]
    pub fn wd(&mut self, ptr: FarPtr, value: FarPtr) {
        self.ww(ptr, off_of(value));
        self.ww(ptr_add(ptr, 2), seg_of(value));
    }

    // Data-segment shorthands.
    #[inline]
    pub fn b(&self, off: u16) -> u8 {
        self.ds[off as usize]
    }
    #[inline]
    pub fn set_b(&mut self, off: u16, value: u8) {
        self.ds[off as usize] = value;
    }
    #[inline]
    pub fn w(&self, off: u16) -> u16 {
        u16::from_le_bytes([self.ds[off as usize], self.ds[off.wrapping_add(1) as usize]])
    }
    #[inline]
    pub fn sw(&self, off: u16) -> i16 {
        self.w(off) as i16
    }
    #[inline]
    pub fn set_w(&mut self, off: u16, value: u16) {
        let [lo, hi] = value.to_le_bytes();
        self.ds[off as usize] = lo;
        self.ds[off.wrapping_add(1) as usize] = hi;
    }
    #[inline]
    pub fn d(&self, off: u16) -> FarPtr {
        self.rd(ds_ptr(off))
    }
    #[inline]
    pub fn set_d(&mut self, off: u16, value: FarPtr) {
        self.wd(ds_ptr(off), value);
    }

    pub fn alloc(&mut self, size: usize) -> FarPtr {
        let block = vec![0u8; size.max(1)];
        let index = if let Some(index) = self.free_slots.pop() {
            self.heap[index] = Some(block);
            index
        } else {
            self.heap.push(Some(block));
            self.heap.len() - 1
        };
        assert!(index < 0xB000, "uk2: heap segment space exhausted");
        far(HEAP_SEG_BASE + index as u16, 0)
    }

    /// `farrealloc` semantics: a null pointer allocates; content is kept.
    pub fn realloc(&mut self, ptr: FarPtr, size: usize) -> FarPtr {
        if ptr == 0 || off_of(ptr) != 0 || self.block(seg_of(ptr)).is_none() {
            return self.alloc(size);
        }
        if let Some(block) = self.block_mut(seg_of(ptr)) {
            block.resize(size.max(1), 0);
        }
        ptr
    }

    pub fn free(&mut self, ptr: FarPtr) {
        let seg = seg_of(ptr);
        if ptr == 0 || seg == DS_SEG || off_of(ptr) != 0 {
            return;
        }
        if let Some(index) = seg.checked_sub(HEAP_SEG_BASE).map(usize::from)
            && self.heap.get(index).is_some_and(Option::is_some)
        {
            self.heap[index] = None;
            self.free_slots.push(index);
        }
    }

    pub fn block_len(&self, ptr: FarPtr) -> usize {
        if seg_of(ptr) == DS_SEG {
            return 0x10000 - off_of(ptr) as usize;
        }
        self.block(seg_of(ptr))
            .map(|block| block.len().saturating_sub(off_of(ptr) as usize))
            .unwrap_or(0)
    }

    /// Whole heap block (or DS tail) starting at `ptr`'s offset; used for
    /// "huge" buffers larger than one segment (VRAM snapshots).
    pub fn slice(&self, ptr: FarPtr) -> &[u8] {
        let off = off_of(ptr) as usize;
        if seg_of(ptr) == DS_SEG {
            return &self.ds[off..];
        }
        self.block(seg_of(ptr))
            .and_then(|block| block.get(off..))
            .unwrap_or(&[])
    }

    pub fn slice_mut(&mut self, ptr: FarPtr) -> &mut [u8] {
        let off = off_of(ptr) as usize;
        if seg_of(ptr) == DS_SEG {
            return &mut self.ds[off..];
        }
        match self
            .block_mut(seg_of(ptr))
            .and_then(|block| block.get_mut(off..))
        {
            Some(slice) => slice,
            None => &mut [],
        }
    }

    pub fn strlen(&self, ptr: FarPtr) -> usize {
        if ptr == 0 {
            return 0;
        }
        let mut n = 0usize;
        while n < 0x10000 && self.rb(ptr_add(ptr, n as i32)) != 0 {
            n += 1;
        }
        n
    }

    pub fn cstr(&self, ptr: FarPtr) -> Vec<u8> {
        let n = self.strlen(ptr);
        self.read_bytes(ptr, n)
    }

    pub fn read_bytes(&self, ptr: FarPtr, len: usize) -> Vec<u8> {
        (0..len).map(|i| self.rb(ptr_add(ptr, i as i32))).collect()
    }

    pub fn write_bytes(&mut self, ptr: FarPtr, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            self.wb(ptr_add(ptr, i as i32), *byte);
        }
    }

    /// `strcpy(dst, bytes)` including the terminator.
    pub fn strcpy_bytes(&mut self, dst: FarPtr, bytes: &[u8]) {
        self.write_bytes(dst, bytes);
        self.wb(ptr_add(dst, bytes.len() as i32), 0);
    }

    pub fn strcpy(&mut self, dst: FarPtr, src: FarPtr) {
        let bytes = self.cstr(src);
        self.strcpy_bytes(dst, &bytes);
    }

    pub fn memcpy(&mut self, dst: FarPtr, src: FarPtr, len: usize) {
        let bytes = self.read_bytes(src, len);
        self.write_bytes(dst, &bytes);
    }

    pub fn memset(&mut self, dst: FarPtr, value: u8, len: usize) {
        for i in 0..len {
            self.wb(ptr_add(dst, i as i32), value);
        }
    }

    /// `sub_20D33`: byte-wise swap of two memory ranges.
    pub fn memswap(&mut self, a: FarPtr, b: FarPtr, len: usize) {
        for i in 0..len {
            let pa = ptr_add(a, i as i32);
            let pb = ptr_add(b, i as i32);
            let va = self.rb(pa);
            let vb = self.rb(pb);
            self.wb(pa, vb);
            self.wb(pb, va);
        }
    }

    /// Allocates a heap block initialised with `bytes`.
    pub fn alloc_bytes(&mut self, bytes: &[u8]) -> FarPtr {
        let ptr = self.alloc(bytes.len());
        self.write_bytes(ptr, bytes);
        ptr
    }

    pub fn alloc_cstr(&mut self, bytes: &[u8]) -> FarPtr {
        let ptr = self.alloc(bytes.len() + 1);
        self.strcpy_bytes(ptr, bytes);
        ptr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn far_pointer_round_trip_and_heap() {
        let mut mem = Mem::blank();
        let p = mem.alloc(8);
        mem.ww(p, 0x1234);
        mem.set_d(0x100, p);
        assert_eq!(mem.d(0x100), p);
        assert_eq!(mem.rw(mem.d(0x100)), 0x1234);
        let p = mem.realloc(p, 32);
        assert_eq!(mem.rw(p), 0x1234);
        mem.free(p);
        assert_eq!(mem.rw(p), 0);
    }
}
