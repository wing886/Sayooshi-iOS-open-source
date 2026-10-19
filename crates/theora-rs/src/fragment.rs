fn clamp255(v: i32) -> u8 {
    v.clamp(0, 255) as u8
}

// The reconstruction loops work on whole 8-pixel rows taken once per row:
// indexing each pixel of the frame checked bounds per pixel and kept the
// compiler from using vector instructions (most of a 1080p OMV's decode).

#[inline(always)]
fn row(frame: &[u8], off: isize) -> &[u8; 8] {
    let at = off as usize;
    frame[at..at + 8].try_into().expect("8-pixel row")
}

#[inline(always)]
fn row_mut(frame: &mut [u8], off: isize) -> &mut [u8; 8] {
    let at = off as usize;
    (&mut frame[at..at + 8]).try_into().expect("8-pixel row")
}

#[inline(always)]
fn residue_row(residue: &[i16; 64], i: usize) -> &[i16; 8] {
    residue[i * 8..i * 8 + 8].try_into().expect("8 residues")
}

fn idx(base: isize, x: isize, ystride: isize) -> usize {
    (base + x * ystride) as usize
}

pub fn oc_frag_copy_c(
    dst_frame: &mut [u8],
    dst_off: isize,
    src_frame: &[u8],
    src_off: isize,
    ystride: isize,
) {
    let mut dst = dst_off;
    let mut src = src_off;
    for _ in 0..8 {
        let d = dst as usize;
        let s = src as usize;
        dst_frame[d..d + 8].copy_from_slice(&src_frame[s..s + 8]);
        dst += ystride;
        src += ystride;
    }
}

pub fn oc_frag_copy_list_c(
    dst_frame: &mut [u8],
    src_frame: &[u8],
    ystride: isize,
    fragis: &[usize],
    frag_buf_offs: &[isize],
) {
    for &fragi in fragis {
        let off = frag_buf_offs[fragi];
        oc_frag_copy_c(dst_frame, off, src_frame, off, ystride);
    }
}

pub fn oc_frag_recon_intra_c(
    dst_frame: &mut [u8],
    dst_off: isize,
    ystride: isize,
    residue: &[i16; 64],
) {
    let mut dst = dst_off;
    for i in 0..8 {
        let r = residue_row(residue, i);
        let d = row_mut(dst_frame, dst);
        for j in 0..8 {
            d[j] = clamp255(r[j] as i32 + 128);
        }
        dst += ystride;
    }
}

pub fn oc_frag_recon_inter_c(
    dst_frame: &mut [u8],
    dst_off: isize,
    src_frame: &[u8],
    src_off: isize,
    ystride: isize,
    residue: &[i16; 64],
) {
    let mut dst = dst_off;
    let mut src = src_off;
    for i in 0..8 {
        let r = residue_row(residue, i);
        let s = row(src_frame, src);
        let d = row_mut(dst_frame, dst);
        for j in 0..8 {
            d[j] = clamp255(r[j] as i32 + s[j] as i32);
        }
        dst += ystride;
        src += ystride;
    }
}

pub fn oc_frag_recon_inter2_c(
    dst_frame: &mut [u8],
    dst_off: isize,
    src1_frame: &[u8],
    src1_off: isize,
    src2_frame: &[u8],
    src2_off: isize,
    ystride: isize,
    residue: &[i16; 64],
) {
    let mut dst = dst_off;
    let mut src1 = src1_off;
    let mut src2 = src2_off;
    for i in 0..8 {
        let r = residue_row(residue, i);
        let s1 = row(src1_frame, src1);
        let s2 = row(src2_frame, src2);
        let d = row_mut(dst_frame, dst);
        for j in 0..8 {
            let pred = (s1[j] as i32 + s2[j] as i32) >> 1;
            d[j] = clamp255(r[j] as i32 + pred);
        }
        dst += ystride;
        src1 += ystride;
        src2 += ystride;
    }
}

pub fn oc_restore_fpu_c() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intra_recon_clamps() {
        let mut dst = vec![0u8; 64];
        let mut residue = [0i16; 64];
        residue[0] = 200;
        residue[1] = -300;
        oc_frag_recon_intra_c(&mut dst, 0, 8, &residue);
        assert_eq!(dst[0], 255);
        assert_eq!(dst[1], 0);
    }
}
