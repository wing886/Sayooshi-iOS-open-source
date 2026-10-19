//! Siglus voice time-compression (`C_jitan_cnv`) port.
//!
//! This intentionally follows the original `util_jitan_cnv.cpp` algorithm
//! instead of using a generic resampler/WSOLA implementation.  Siglus keeps
//! the sample rate unchanged, copies a short block, skips a rate-dependent
//! block, then searches/smooths the splice against the preceding waveform.

use anyhow::{Context, Result, bail, ensure};
use siglus_assets::vorbis::{Pcm16, pcm16_to_wav_bytes};

const JITAN_RATE_NORMAL: i32 = 100;
const JITAN_RATE_CONVERTER_MAX: i32 = 400;
const JITAN_CLAMP_SAMPLE: i32 = 32_760;
const JITAN_STEREO_SUM_CLAMP: i32 = 30_000;

/// Apply the original Siglus JITAN path to a decoded PCM16 WAV.
///
/// `elm_sound_player.cpp` only invokes the converter when the requested rate
/// differs from 100%.  At JITAN rates, stereo KOE is first converted to mono by
/// summing L+R and clamping to +/-30000.  The low-level converter itself only
/// accepts mono 16-bit PCM.
pub fn convert_koe_wav(wav: Vec<u8>, percent: u16) -> Result<Vec<u8>> {
    if percent == JITAN_RATE_NORMAL as u16 {
        return Ok(wav);
    }

    let pcm = parse_pcm16_wav(&wav)?;
    let mono = match pcm.channels {
        1 => pcm.samples,
        2 => stereo_to_mono_original(&pcm.samples),
        channels => {
            bail!(
                "Siglus JITAN only handles mono/stereo KOE before the mono converter; got {channels} channels"
            )
        }
    };

    let converted = convert_pcm16_original(&mono, pcm.sample_rate, percent as i32)
        // C_sound::create() zero-initializes the destination vector.  The
        // original caller ignores C_jitan_cnv::convert()'s false return, so a
        // voice shorter than 100 ms is played as a same-length zero buffer.
        .unwrap_or_else(|| vec![0; mono.len()]);

    Ok(pcm16_to_wav_bytes(&Pcm16 {
        channels: 1,
        sample_rate: pcm.sample_rate,
        samples: converted,
    }))
}

fn parse_pcm16_wav(wav: &[u8]) -> Result<Pcm16> {
    ensure!(
        wav.len() >= 12 && &wav[..4] == b"RIFF" && &wav[8..12] == b"WAVE",
        "KOE JITAN requires RIFF/WAVE"
    );

    let mut pos = 12usize;
    let mut format: Option<(u16, u16, u32, u16)> = None;
    let mut data: Option<&[u8]> = None;
    while pos + 8 <= wav.len() {
        let tag = &wav[pos..pos + 4];
        let size = u32::from_le_bytes(
            wav[pos + 4..pos + 8]
                .try_into()
                .context("read WAV chunk size")?,
        ) as usize;
        pos += 8;
        let end = pos.checked_add(size).context("WAV chunk size overflow")?;
        ensure!(end <= wav.len(), "truncated WAV chunk");

        if tag == b"fmt " {
            ensure!(size >= 16, "short WAV fmt chunk");
            format = Some((
                u16::from_le_bytes(wav[pos..pos + 2].try_into()?),
                u16::from_le_bytes(wav[pos + 2..pos + 4].try_into()?),
                u32::from_le_bytes(wav[pos + 4..pos + 8].try_into()?),
                u16::from_le_bytes(wav[pos + 14..pos + 16].try_into()?),
            ));
        } else if tag == b"data" && data.is_none() {
            data = Some(&wav[pos..end]);
        }

        pos = end.saturating_add(size & 1);
    }

    let (format_tag, channels, sample_rate, bits) = format.context("missing WAV fmt chunk")?;
    ensure!(
        format_tag == 1,
        "JITAN requires PCM WAV, got format {format_tag}"
    );
    ensure!(bits == 16, "JITAN requires 16-bit PCM, got {bits}-bit");
    ensure!(channels != 0, "JITAN WAV has zero channels");
    ensure!(sample_rate != 0, "JITAN WAV has zero sample rate");
    let data = data.context("missing WAV data chunk")?;
    let frame_bytes = channels as usize * 2;
    ensure!(
        data.len() % frame_bytes == 0,
        "unaligned PCM16 KOE payload: {} bytes / {} channels",
        data.len(),
        channels
    );

    let samples = data
        .as_chunks::<2>()
        .0
        .iter()
        .map(|sample| i16::from_le_bytes([sample[0], sample[1]]))
        .collect();
    Ok(Pcm16 {
        channels,
        sample_rate,
        samples,
    })
}

fn stereo_to_mono_original(input: &[i16]) -> Vec<i16> {
    input
        .as_chunks::<2>()
        .0
        .iter()
        .map(|pair| {
            (pair[0] as i32 + pair[1] as i32).clamp(-JITAN_STEREO_SUM_CLAMP, JITAN_STEREO_SUM_CLAMP)
                as i16
        })
        .collect()
}

/// Safe index-based transcription of `C_jitan_cnv::convert()` and
/// `convert_func()`.  `None` is the original `false` return.
fn convert_pcm16_original(input: &[i16], sample_rate: u32, percent: i32) -> Option<Vec<i16>> {
    if input.is_empty() {
        return None;
    }

    let percent = percent.clamp(JITAN_RATE_NORMAL, JITAN_RATE_CONVERTER_MAX);
    let extra_percent = percent - JITAN_RATE_NORMAL;
    let rate = usize::try_from(sample_rate).ok()?;
    let play_block_smp = rate / 80;
    let before_smp = rate / 110;
    let after_smp = rate / 110;
    let delete_block_smp = play_block_smp.saturating_mul(extra_percent as usize) / 100;

    convert_func_original(
        input,
        rate,
        play_block_smp,
        delete_block_smp,
        before_smp,
        after_smp,
        true,
        true,
    )
}

fn convert_func_original(
    input: &[i16],
    rate: usize,
    play_block_smp: usize,
    delete_block_smp: usize,
    before_smp: usize,
    after_smp: usize,
    zero_flag: bool,
    smooth_flag: bool,
) -> Option<Vec<i16>> {
    // Original: wave_size < ((rate / 10) << 1) => false for mono PCM16.
    if input.len() < rate / 10 || play_block_smp == 0 {
        return None;
    }

    let work_base_smp = play_block_smp;
    let jump_smp = delete_block_smp;

    // C++ reserves the original PCM bytes plus one second.  Its helper bounds
    // still use the original wave_size, so keep that separate from capacity.
    let dst_bound = input.len();
    let mut output = vec![0i16; input.len().saturating_add(rate)];
    let center = before_smp;
    let mut splice = vec![0i16; before_smp.saturating_add(after_smp)];
    let mut splice_before_count = 0usize;
    let mut splice_after_count = 0usize;

    let mut dst = 0usize;
    let mut src = 0usize;
    let mut new_sample_count = 0usize;
    let mut before_copy_smp = 0usize;

    loop {
        if src >= input.len() {
            break;
        }

        if dst != 0 && zero_flag {
            src = convert_func_16bit_rep(&output, dst - 1, input, src, work_base_smp, dst_bound);
        }
        if src >= input.len() {
            break;
        }

        let mut copy_smp = if zero_flag {
            convert_func_16bit_copy_size(input, src, work_base_smp)
        } else {
            work_base_smp
        };
        copy_smp = copy_smp.min(input.len() - src);

        splice_before_count = before_smp.min(src);
        if splice_before_count != 0 {
            let from = src - splice_before_count;
            let to = center;
            splice[to - splice_before_count..to].copy_from_slice(&input[from..src]);
        }

        if output.len() < dst.saturating_add(copy_smp) {
            output.resize(dst.saturating_add(copy_smp), 0);
        }
        if copy_smp != 0 {
            output[dst..dst + copy_smp].copy_from_slice(&input[src..src + copy_smp]);
        }

        let mut smooth_len = 0i32;
        if dst != 0 && smooth_flag {
            smooth_len =
                convert_func_smooth(&mut output, dst, before_copy_smp, copy_smp, dst_bound);
        }
        before_copy_smp = copy_smp;

        if splice_before_count != 0 || splice_after_count != 0 {
            convert_func_gousei(
                &mut output,
                dst,
                dst_bound,
                &mut splice,
                center,
                splice_before_count,
                splice_after_count,
                smooth_len,
            );
        }

        splice_after_count = after_smp.min(input.len().saturating_sub(src + copy_smp));
        if splice_after_count != 0 {
            splice[center..center + splice_after_count]
                .copy_from_slice(&input[src + copy_smp..src + copy_smp + splice_after_count]);
        }

        dst = dst.saturating_add(copy_smp);
        src = src.saturating_add(copy_smp);
        new_sample_count = new_sample_count.saturating_add(copy_smp);
        src = src.saturating_add(jump_smp);
    }

    if new_sample_count >= 100 {
        let start = new_sample_count - 100;
        for offset in 0..100usize {
            let multiplier = 100i32 - offset as i32;
            output[start + offset] = ((output[start + offset] as i32 * multiplier) / 100) as i16;
        }
    }

    output.truncate(new_sample_count);
    Some(output)
}

fn convert_func_16bit_rep(
    dst: &[i16],
    dst_last: usize,
    src: &[i16],
    src_index: usize,
    work_base_smp: usize,
    dst_bound: usize,
) -> usize {
    let limit_smp_cnt1 = work_base_smp >> 2;
    let limit_smp_cnt2 = work_base_smp >> 2;
    if limit_smp_cnt1 == 0 || limit_smp_cnt2 == 0 {
        return src_index;
    }

    let (vector_flag, target_smp) = if dst_last < dst_bound && dst_last < dst.len() {
        (
            convert_func_16bit_vector(dst, dst_last, dst_bound),
            dst[dst_last] as i32,
        )
    } else {
        (0, 0)
    };

    if vector_flag != 0 {
        if let Some(distance) = convert_func_16bit_rep_func(
            src,
            src_index,
            limit_smp_cnt1,
            limit_smp_cnt2,
            vector_flag,
            target_smp,
            -1,
        ) {
            return src_index.saturating_sub(distance);
        }
        if let Some(distance) = convert_func_16bit_rep_func(
            src,
            src_index,
            limit_smp_cnt1,
            limit_smp_cnt2,
            vector_flag,
            target_smp,
            1,
        ) {
            return src_index.saturating_add(distance).min(src.len());
        }
    }

    if let Some(distance) = convert_func_16bit_rep_func(
        src,
        src_index,
        limit_smp_cnt1,
        limit_smp_cnt2,
        0,
        target_smp,
        -1,
    ) {
        return src_index.saturating_sub(distance);
    }
    if let Some(distance) = convert_func_16bit_rep_func(
        src,
        src_index,
        limit_smp_cnt1,
        limit_smp_cnt2,
        0,
        target_smp,
        1,
    ) {
        return src_index.saturating_add(distance).min(src.len());
    }
    src_index
}

fn convert_func_16bit_rep_func(
    src: &[i16],
    start: usize,
    limit_smp_cnt1: usize,
    limit_smp_cnt2: usize,
    vector_flag: i32,
    target_smp: i32,
    add: isize,
) -> Option<usize> {
    let mut index = start as isize;
    let mut best_count: Option<usize> = None;
    let mut test_smp_cnt = 0usize;
    let mut get_flag = false;
    let mut min_len = 999_999i32;

    loop {
        if index < 0 || index as usize >= src.len() {
            break;
        }
        let idx = index as usize;
        let sample = src[idx] as i32;

        if vector_flag == 0 {
            let distance = (target_smp - sample).abs();
            if distance < min_len {
                min_len = distance;
                best_count = Some(test_smp_cnt);
            }
        } else {
            let candidate_vector = convert_func_16bit_vector(src, idx, src.len());
            if candidate_vector == vector_flag {
                if vector_flag == 1 {
                    if sample <= target_smp {
                        let distance = target_smp - sample;
                        if !get_flag || distance < min_len {
                            min_len = distance;
                            best_count = Some(test_smp_cnt);
                            get_flag = true;
                        }
                    } else if !get_flag {
                        let distance = sample - target_smp;
                        if distance < min_len {
                            min_len = distance;
                            best_count = Some(test_smp_cnt);
                        }
                    }
                } else if sample >= target_smp {
                    let distance = sample - target_smp;
                    if !get_flag || distance < min_len {
                        min_len = distance;
                        best_count = Some(test_smp_cnt);
                        get_flag = true;
                    }
                } else if !get_flag {
                    let distance = target_smp - sample;
                    if distance < min_len {
                        min_len = distance;
                        best_count = Some(test_smp_cnt);
                    }
                }
            }
        }

        if best_count.is_some() && min_len <= 100 {
            break;
        }
        test_smp_cnt += 1;
        index += add;
        if test_smp_cnt >= limit_smp_cnt1 {
            break;
        }
        if test_smp_cnt >= limit_smp_cnt2
            && if vector_flag == 0 {
                best_count.is_some()
            } else {
                get_flag
            }
        {
            break;
        }
    }

    best_count
}

fn convert_func_16bit_vector(src: &[i16], index: usize, end: usize) -> i32 {
    if index >= end || index >= src.len() {
        return 0;
    }
    let base_smp = src[index] as i32;
    let mut vector_flag = 0i32;
    let mut wp = index as isize - 1;
    for _ in 0..10 {
        if wp < 0 || wp as usize >= end || wp as usize >= src.len() {
            break;
        }
        let sample = src[wp as usize] as i32;
        if sample < base_smp {
            vector_flag -= 1;
            if vector_flag <= -3 {
                break;
            }
        } else if sample > base_smp {
            vector_flag += 1;
            if vector_flag >= 3 {
                break;
            }
        }
        wp -= 1;
    }

    if vector_flag >= 3 {
        1
    } else if vector_flag <= -3 {
        -1
    } else {
        0
    }
}

fn convert_func_16bit_copy_size(src: &[i16], src_index: usize, work_base_smp: usize) -> usize {
    let remaining = src.len().saturating_sub(src_index);
    if remaining <= work_base_smp {
        return remaining;
    }

    let mut copy_smp = work_base_smp;
    let mut wp = src_index + copy_smp - 1;
    let mut min_smp = (src[wp] as i32).abs();
    wp = wp.saturating_sub(1);
    let mut min_i = 0usize;
    for i in 0..10usize {
        if wp < src_index || wp >= src.len() {
            break;
        }
        let test_smp = (src[wp] as i32).abs();
        if test_smp < min_smp {
            min_smp = test_smp;
            min_i = i + 1;
        }
        if wp == 0 {
            break;
        }
        wp -= 1;
    }
    if min_i != 0 {
        copy_smp = copy_smp.saturating_sub(min_i);
    }

    let boundary = src_index + copy_smp - 1;
    min_smp = (src[boundary] as i32).abs();
    let sign_negative = src[boundary] < 0;

    if min_smp == 0 {
        return copy_smp;
    }

    let limit_smp_cnt = copy_smp >> 2;

    let mut min_smp_a = min_smp;
    let mut copy_smp_cnt_a = 0usize;
    let mut test_smp_cnt = 1usize;
    let mut scan = src_index + copy_smp;
    loop {
        if scan >= src.len() {
            break;
        }
        let raw = src[scan];
        if (raw < 0) != sign_negative {
            break;
        }
        let test_smp = (raw as i32).abs();
        if test_smp < min_smp_a {
            copy_smp_cnt_a = test_smp_cnt;
            min_smp_a = test_smp;
            if min_smp_a == 0 {
                break;
            }
        }
        test_smp_cnt += 1;
        if test_smp_cnt >= limit_smp_cnt {
            break;
        }
        scan += 1;
    }

    let mut min_smp_b = min_smp;
    let mut copy_smp_cnt_b = 0usize;
    test_smp_cnt = 1;
    let mut scan = src_index as isize + copy_smp as isize - 2;
    loop {
        if scan < src_index as isize || scan < 0 || scan as usize >= src.len() {
            break;
        }
        let raw = src[scan as usize];
        if (raw < 0) != sign_negative {
            break;
        }
        let test_smp = (raw as i32).abs();
        if test_smp < min_smp_b {
            copy_smp_cnt_b = test_smp_cnt;
            min_smp_b = test_smp;
            if min_smp_b == 0 {
                break;
            }
        }
        test_smp_cnt += 1;
        if test_smp_cnt >= limit_smp_cnt {
            break;
        }
        scan -= 1;
    }

    if min_smp_a <= min_smp_b {
        copy_smp.saturating_add(copy_smp_cnt_a).min(remaining)
    } else {
        copy_smp.saturating_sub(copy_smp_cnt_b)
    }
}

fn convert_func_smooth(
    dst: &mut [i16],
    boundary: usize,
    before_smp_cnt: usize,
    after_smp_cnt: usize,
    end: usize,
) -> i32 {
    if boundary == 0 || boundary >= end || boundary >= dst.len() {
        return 0;
    }

    let smooth_len = (dst[boundary] as i32 - dst[boundary - 1] as i32) >> 1;

    if before_smp_cnt != 0 {
        let mut wp = boundary as isize - 1;
        for i in (1..=before_smp_cnt).rev() {
            if wp < 0 || wp as usize >= end || wp as usize >= dst.len() {
                break;
            }
            let correction = ((smooth_len as f64 / before_smp_cnt as f64) * i as f64) as i32;
            if correction == 0 {
                break;
            }
            let value = (dst[wp as usize] as i32 + correction)
                .clamp(-JITAN_CLAMP_SAMPLE, JITAN_CLAMP_SAMPLE);
            dst[wp as usize] = value as i16;
            wp -= 1;
        }
    }

    if after_smp_cnt != 0 {
        let mut wp = boundary;
        for i in (1..=after_smp_cnt).rev() {
            if wp >= end || wp >= dst.len() {
                break;
            }
            let correction = ((smooth_len as f64 / after_smp_cnt as f64) * i as f64) as i32;
            if correction == 0 {
                break;
            }
            let value =
                (dst[wp] as i32 - correction).clamp(-JITAN_CLAMP_SAMPLE, JITAN_CLAMP_SAMPLE);
            dst[wp] = value as i16;
            wp += 1;
        }
    }

    smooth_len
}

#[allow(clippy::too_many_arguments)]
fn convert_func_gousei(
    dst: &mut [i16],
    boundary: usize,
    end: usize,
    splice: &mut [i16],
    center: usize,
    mut before_count: usize,
    mut after_count: usize,
    smooth_len: i32,
) {
    if before_count != 0 {
        if smooth_len != 0 {
            let mut dp = center as isize - 1;
            for _ in 0..before_count {
                if dp < 0 || dp as usize >= splice.len() {
                    break;
                }
                let value = (splice[dp as usize] as i32 - smooth_len)
                    .clamp(-JITAN_CLAMP_SAMPLE, JITAN_CLAMP_SAMPLE);
                splice[dp as usize] = value as i16;
                dp -= 1;
            }
        }
        before_count = before_count.min(boundary);
    }

    if after_count != 0 {
        if smooth_len != 0 {
            let mut dp = center;
            for _ in 0..after_count {
                if dp >= splice.len() {
                    break;
                }
                let value =
                    (splice[dp] as i32 + smooth_len).clamp(-JITAN_CLAMP_SAMPLE, JITAN_CLAMP_SAMPLE);
                splice[dp] = value as i16;
                dp += 1;
            }
        }
        after_count = if boundary >= end {
            0
        } else {
            after_count.min(end - boundary)
        };
    }

    if after_count != 0 {
        for i in 0..after_count {
            if boundary + i >= dst.len() || center + i >= splice.len() {
                break;
            }
            std::mem::swap(&mut dst[boundary + i], &mut splice[center + i]);
        }
    }

    let proc_count = before_count.saturating_add(after_count);
    if proc_count <= 1 {
        return;
    }

    let aaa = (proc_count - 1) as f64;
    let splice_start = center.saturating_sub(before_count);
    let dst_start = boundary.saturating_sub(before_count);
    for i in 0..proc_count {
        if splice_start + i >= splice.len() || dst_start + i >= dst.len() {
            break;
        }
        let value = ((splice[splice_start + i] as f64 / aaa) * i as f64
            + (dst[dst_start + i] as f64 / aaa) * ((proc_count - 1 - i) as f64))
            as i32;
        dst[dst_start + i] = value.clamp(-JITAN_CLAMP_SAMPLE, JITAN_CLAMP_SAMPLE) as i16;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn wav(channels: u16, sample_rate: u32, samples: Vec<i16>) -> Vec<u8> {
        pcm16_to_wav_bytes(&Pcm16 {
            channels,
            sample_rate,
            samples,
        })
    }

    #[test]
    fn normal_rate_is_byte_identical() {
        let input = wav(2, 8_000, vec![100, -100, 200, -200]);
        assert_eq!(convert_koe_wav(input.clone(), 100).unwrap(), input);
    }

    #[test]
    fn stereo_jitan_uses_original_sum_to_mono_rule() {
        // 80 ms: C_jitan_cnv rejects it after the stereo->mono conversion.
        // The original caller ignores that false return and plays the newly
        // created zero-filled mono C_sound buffer.
        let frames = 8_000usize * 80 / 1000;
        let mut samples = Vec::with_capacity(frames * 2);
        for _ in 0..frames {
            samples.push(20_000);
            samples.push(20_000);
        }
        let out = convert_koe_wav(wav(2, 8_000, samples), 200).unwrap();
        let pcm = parse_pcm16_wav(&out).unwrap();
        assert_eq!(pcm.channels, 1);
        assert_eq!(pcm.samples.len(), frames);
        assert!(pcm.samples.iter().all(|&sample| sample == 0));
    }

    fn fnv1a_pcm16(samples: &[i16]) -> u64 {
        let mut hash = 0xcbf29ce484222325u64;
        for sample in samples {
            for byte in sample.to_le_bytes() {
                hash ^= byte as u64;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
        hash
    }

    #[test]
    fn converter_matches_original_cpp_golden_vectors() {
        // Golden values produced by the original util_jitan_cnv.cpp with the
        // same deterministic mono PCM input.  This catches seemingly harmless
        // changes to splice search, smoothing, integer rounding, or tail fade.
        let input = (0..8_000usize)
            .map(|i| (((i * 7_919 + 12_345) % 60_001) as i32 - 30_000) as i16)
            .collect::<Vec<_>>();
        for (rate, expected_len, expected_hash) in [
            (125, 6_526, 0x32ed7b7901ceab1a),
            (150, 5_754, 0xcb55646dcdd69270),
            (200, 4_130, 0x7422c527b9b4de3d),
            (300, 2_599, 0x88033d2b78a75beb),
        ] {
            let out = convert_pcm16_original(&input, 8_000, rate).unwrap();
            assert_eq!(out.len(), expected_len, "rate={rate}");
            assert_eq!(fnv1a_pcm16(&out), expected_hash, "rate={rate}");
        }
    }

    #[test]
    fn original_converter_shortens_long_voice_and_fades_tail() {
        let rate = 8_000u32;
        let frames = rate as usize;
        let samples = (0..frames)
            .map(|i| {
                ((i as f64 * 440.0 * std::f64::consts::TAU / rate as f64).sin() * 12_000.0) as i16
            })
            .collect::<Vec<_>>();
        let out = convert_koe_wav(wav(1, rate, samples), 200).unwrap();
        let pcm = parse_pcm16_wav(&out).unwrap();
        assert_eq!(pcm.channels, 1);
        assert!(pcm.samples.len() < frames);
        assert!(pcm.samples.len() > frames / 3);
        assert!(pcm.samples.last().copied().unwrap_or(0).abs() < 500);
    }
}
