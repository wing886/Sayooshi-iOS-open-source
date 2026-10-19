//! UK2 `MMD` / `MMM` music resource wrappers.
//!
//! The score bytes are preserved while the outer file layouts are parsed for
//! resource verification and further driver-format reverse engineering.

use anyhow::{Context, Result, bail};
use std::ops::Range;
use std::path::Path;

const MMM_TRACK_COUNT: usize = 13;
const MMM_TRACK_TABLE_BYTES: usize = MMM_TRACK_COUNT * 2;
const MMD_TRACK_COUNT: usize = 18;
const MMD_TRACK_TABLE_START: usize = 2;
const MMD_TRACK_TABLE_BYTES: usize = MMD_TRACK_COUNT * 4;
const MMD_DIRECTORY_END: usize = MMD_TRACK_TABLE_START + MMD_TRACK_TABLE_BYTES;
const MMD_TITLE_OFFSET: usize = 0x50;

/// One channel stream in a UK2 `.MMM` music resource.
///
/// The table stores big-endian absolute offsets. Values before the end of the
/// table are empty-channel markers found in the shipped files; equal offsets
/// are valid and mean that two channels share a stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmmTrack {
    pub index: usize,
    pub offset: Option<usize>,
    pub data: Range<usize>,
}

/// One of the 18 channel/control streams in a KAJA `.MMD` score.
///
/// The directory stores a little-endian absolute offset, a key transpose byte,
/// and a MIDI channel byte. Channel `0xff` denotes one of the driver control
/// streams.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MmdTrack {
    pub slot: usize,
    pub channel: Option<u8>,
    pub key: u8,
    pub offset: usize,
    pub data: Range<usize>,
}

/// One expanded four-byte KAJA MMD command record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MmdRecord {
    pub bytes: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Uk2MusicKind {
    Mmd,
    Mmm,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Uk2MusicFile {
    kind: Uk2MusicKind,
    bytes: Vec<u8>,
    mmd_tempo: Option<u8>,
    mmd_key: Option<u8>,
    mmd_title_end: Option<usize>,
    mmd_tracks: Vec<MmdTrack>,
    mmm_tracks: Vec<MmmTrack>,
}

impl Uk2MusicFile {
    pub fn from_path(kind: Uk2MusicKind, path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let bytes = std::fs::read(path)
            .with_context(|| format!("failed to read UK2 music {}", path.display()))?;
        Self::from_bytes(kind, bytes)
    }

    pub fn from_bytes(kind: Uk2MusicKind, bytes: Vec<u8>) -> Result<Self> {
        if bytes.is_empty() {
            bail!("uk2: music resource is empty");
        }
        let (mmd_tempo, mmd_key, mmd_title_end, mmd_tracks, mmm_tracks) = match kind {
            Uk2MusicKind::Mmd => {
                let (tempo, key, title_end, tracks) = parse_mmd_tracks(&bytes)?;
                (Some(tempo), Some(key), Some(title_end), tracks, Vec::new())
            }
            Uk2MusicKind::Mmm => (None, None, None, Vec::new(), parse_mmm_tracks(&bytes)?),
        };
        Ok(Self {
            kind,
            bytes,
            mmd_tempo,
            mmd_key,
            mmd_title_end,
            mmd_tracks,
            mmm_tracks,
        })
    }

    pub fn kind(&self) -> Uk2MusicKind {
        self.kind
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// The 13 channel entries in an `.MMM` score. MMD resources use a separate
    /// driver format and return an empty slice here.
    pub fn mmm_tracks(&self) -> &[MmmTrack] {
        &self.mmm_tracks
    }

    /// MMD score tempo from byte 0.
    pub fn mmd_tempo(&self) -> Option<u8> {
        self.mmd_tempo
    }

    /// Global key/transpose value from byte 1.
    pub fn mmd_key(&self) -> Option<u8> {
        self.mmd_key
    }

    /// Raw title bytes from offset `0x50` through the NUL terminator.
    pub fn mmd_title(&self) -> Option<&[u8]> {
        let end = self.mmd_title_end?;
        self.bytes.get(MMD_TITLE_OFFSET..end)
    }

    /// The 18 MMD channel/control streams. MMM resources return an empty slice.
    pub fn mmd_tracks(&self) -> &[MmdTrack] {
        &self.mmd_tracks
    }

    /// Bytes for an MMD channel/control stream. MMD events are still encoded
    /// in KAJA's driver format; this only returns its bounded directory slice.
    pub fn mmd_track_bytes(&self, slot: usize) -> Option<&[u8]> {
        let track = self.mmd_tracks.get(slot)?;
        self.bytes.get(track.data.clone())
    }

    /// Expand the delta-compressed records in an MMD track through its `0xfe`
    /// end marker. The record bytes retain the driver's command encoding.
    pub fn decode_mmd_track(&self, slot: usize) -> Result<Vec<MmdRecord>> {
        if self.kind != Uk2MusicKind::Mmd {
            bail!("uk2: cannot decode an MMD track from an MMM resource");
        }
        let bytes = self
            .mmd_track_bytes(slot)
            .with_context(|| format!("MMD track {slot} does not exist"))?;
        decode_mmd_records(bytes).with_context(|| format!("failed to decode MMD track {slot}"))
    }

    /// Bytes for a parsed MMM channel. Returns `None` for an empty marker or
    /// for MMD resources.
    pub fn mmm_track_bytes(&self, track: usize) -> Option<&[u8]> {
        let track = self.mmm_tracks.get(track)?;
        track.offset?;
        self.bytes.get(track.data.clone())
    }

    pub fn words_le(&self) -> impl Iterator<Item = u16> + '_ {
        self.bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
    }

    pub fn words_be(&self) -> impl Iterator<Item = u16> + '_ {
        self.bytes
            .chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
    }
}

fn decode_mmd_records(bytes: &[u8]) -> Result<Vec<MmdRecord>> {
    let end = bytes
        .iter()
        .position(|byte| *byte == 0xfe)
        .context("MMD track has no 0xfe end marker")?;
    let bytes = &bytes[..end];
    let mut records = Vec::new();
    let mut cursor = 0;
    let mut previous = [0_u8; 4];
    let mut sysex = false;

    while cursor < bytes.len() {
        if sysex {
            if bytes.len() - cursor < 2 {
                bail!("truncated MMD SysEx escape at byte {cursor:#x}");
            }
            previous[2] = bytes[cursor];
            previous[3] = bytes[cursor + 1];
            cursor += 2;
            records.push(MmdRecord { bytes: previous });
            sysex = previous[3] != 0xf7;
            continue;
        }

        let lead = bytes[cursor];
        if lead == 0x98 {
            previous[0] = lead;
            cursor += 1;
            sysex = true;
            records.push(MmdRecord { bytes: previous });
            continue;
        } else if lead & 0xf0 == 0x80 {
            cursor += 1;
            let mask = (lead & 0x0f) << 4;
            for (index, bit) in [0x80, 0x40, 0x20, 0x10].into_iter().enumerate() {
                if mask & bit != 0 {
                    let Some(&value) = bytes.get(cursor) else {
                        bail!("truncated MMD delta record at byte {cursor:#x}");
                    };
                    previous[index] = value;
                    cursor += 1;
                }
            }
        } else {
            let available = (bytes.len() - cursor).min(4);
            if available < 4 && cursor + available != bytes.len() {
                bail!("truncated MMD record at byte {cursor:#x}");
            }
            previous = [0; 4];
            previous[..available].copy_from_slice(&bytes[cursor..cursor + available]);
            cursor += available;
        }
        records.push(MmdRecord { bytes: previous });
    }

    Ok(records)
}

fn parse_mmd_tracks(bytes: &[u8]) -> Result<(u8, u8, usize, Vec<MmdTrack>)> {
    if bytes.len() < MMD_DIRECTORY_END || bytes.len() <= MMD_TITLE_OFFSET {
        bail!(
            "uk2: MMD resource is {} bytes, shorter than its track directory or title field",
            bytes.len(),
        );
    }
    let tempo = bytes[0];
    let key = bytes[1];
    if tempo == 0 {
        bail!("uk2: MMD tempo is zero");
    }

    let mut title_end = MMD_TITLE_OFFSET;
    while title_end < bytes.len() && bytes[title_end] != 0 {
        title_end += 1;
    }
    if title_end == bytes.len() {
        bail!("uk2: MMD title at {MMD_TITLE_OFFSET:#x} is not NUL terminated");
    }
    let title_data_end = title_end + 1;

    let mut directory = Vec::with_capacity(MMD_TRACK_COUNT);
    let mut previous_offset = 0usize;
    for slot in 0..MMD_TRACK_COUNT {
        let start = MMD_TRACK_TABLE_START + slot * 4;
        let offset = usize::from(u16::from_le_bytes([bytes[start], bytes[start + 1]]));
        let key = bytes[start + 2];
        let channel_id = bytes[start + 3];
        if offset < title_data_end || offset >= bytes.len() {
            bail!("uk2: MMD track {slot} offset {offset:#x} is outside the score data");
        }
        if offset < previous_offset {
            bail!("uk2: MMD track {slot} offset {offset:#x} is not monotonic");
        }
        previous_offset = offset;
        let channel = if channel_id == 0xff {
            None
        } else if channel_id < 16 {
            Some(channel_id)
        } else {
            bail!("uk2: MMD track {slot} has invalid MIDI channel {channel_id:#04x}");
        };
        directory.push((channel, key, offset));
    }

    let mut tracks = Vec::with_capacity(MMD_TRACK_COUNT);
    for (slot, (channel, key, offset)) in directory.iter().copied().enumerate() {
        let end = directory
            .iter()
            .map(|(_, _, candidate)| *candidate)
            .filter(|candidate| *candidate > offset)
            .min()
            .unwrap_or(bytes.len());
        tracks.push(MmdTrack {
            slot,
            channel,
            key,
            offset,
            data: offset..end,
        });
    }
    if directory[0].2 != title_data_end {
        bail!(
            "uk2: MMD first track starts at {:#x}, after title terminator {title_data_end:#x}",
            directory[0].2
        );
    }
    Ok((tempo, key, title_end, tracks))
}

fn parse_mmm_tracks(bytes: &[u8]) -> Result<Vec<MmmTrack>> {
    if bytes.len() < MMM_TRACK_TABLE_BYTES {
        bail!(
            "uk2: MMM resource is {} bytes, shorter than its {MMM_TRACK_COUNT}-entry offset table",
            bytes.len()
        );
    }

    let offsets = bytes[..MMM_TRACK_TABLE_BYTES]
        .chunks_exact(2)
        .map(|chunk| usize::from(u16::from_be_bytes([chunk[0], chunk[1]])))
        .collect::<Vec<_>>();

    // A value inside the header is an empty-channel marker, not a byte offset.
    // Real score data starts after the 26-byte directory. Do not require the
    // table order to be monotonic: the game files store channels independently.
    let mut tracks = Vec::with_capacity(MMM_TRACK_COUNT);
    for (index, offset) in offsets.iter().copied().enumerate() {
        if offset < MMM_TRACK_TABLE_BYTES || offset >= bytes.len() {
            tracks.push(MmmTrack {
                index,
                offset: None,
                data: 0..0,
            });
            continue;
        }
        let end = offsets
            .iter()
            .copied()
            .filter(|candidate| *candidate > offset && *candidate < bytes.len())
            .min()
            .unwrap_or(bytes.len());
        tracks.push(MmmTrack {
            index,
            offset: Some(offset),
            data: offset..end,
        });
    }
    Ok(tracks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mmd_header_directory_and_title() {
        let mut bytes = vec![0; 96];
        bytes[0] = 120;
        bytes[1] = 0;
        bytes[MMD_TITLE_OFFSET..MMD_TITLE_OFFSET + 5].copy_from_slice(b"Test\0");
        for slot in 0..MMD_TRACK_COUNT {
            let start = MMD_TRACK_TABLE_START + slot * 4;
            bytes[start..start + 2].copy_from_slice(&85_u16.to_le_bytes());
            bytes[start + 2] = slot as u8;
            bytes[start + 3] = if slot < 16 { slot as u8 } else { 0xff };
        }
        bytes[85..].copy_from_slice(&[1, 2, 3, 4, 0xfe, 0, 0, 0, 0, 0, 0]);
        let music = Uk2MusicFile::from_bytes(Uk2MusicKind::Mmd, bytes).unwrap();
        assert_eq!(music.kind(), Uk2MusicKind::Mmd);
        assert_eq!(music.mmd_tempo(), Some(120));
        assert_eq!(music.mmd_key(), Some(0));
        assert_eq!(music.mmd_title(), Some(&b"Test"[..]));
        assert_eq!(music.mmd_tracks().len(), MMD_TRACK_COUNT);
        assert_eq!(music.mmd_tracks()[9].channel, Some(9));
        assert_eq!(music.mmd_tracks()[9].key, 9);
        assert_eq!(music.mmd_tracks()[16].channel, None);
        assert_eq!(
            music.mmd_track_bytes(0),
            Some(&[1, 2, 3, 4, 0xfe, 0, 0, 0, 0, 0, 0][..])
        );
        assert_eq!(music.words_le().next(), Some(120));
    }

    #[test]
    fn rejects_mmd_directory_offsets_outside_the_resource() {
        let mut bytes = vec![0; 90];
        bytes[0] = 120;
        bytes[MMD_TITLE_OFFSET..MMD_TITLE_OFFSET + 2].copy_from_slice(b"T\0");
        for slot in 0..MMD_TRACK_COUNT {
            let start = MMD_TRACK_TABLE_START + slot * 4;
            let offset = if slot == 3 { 100_u16 } else { 82_u16 };
            bytes[start..start + 2].copy_from_slice(&offset.to_le_bytes());
            bytes[start + 3] = if slot < 16 { slot as u8 } else { 0xff };
        }
        assert!(Uk2MusicFile::from_bytes(Uk2MusicKind::Mmd, bytes).is_err());
    }

    #[test]
    fn parses_big_endian_mmm_channel_offsets_without_requiring_sorted_table() {
        let mut bytes = vec![0; 40];
        let offsets = [26u16, 34, 30, 4, 34, 39, 2, 40, 26, 31, 35, 33, 28];
        for (index, offset) in offsets.into_iter().enumerate() {
            bytes[index * 2..index * 2 + 2].copy_from_slice(&offset.to_be_bytes());
        }
        let music = Uk2MusicFile::from_bytes(Uk2MusicKind::Mmm, bytes.clone()).unwrap();
        assert_eq!(music.mmm_tracks().len(), MMM_TRACK_COUNT);
        assert_eq!(music.mmm_track_bytes(0), Some(&bytes[26..28]));
        assert_eq!(music.mmm_track_bytes(1), Some(&bytes[34..35]));
        assert_eq!(music.mmm_track_bytes(3), None);
        assert_eq!(music.mmm_track_bytes(7), None);
    }

    #[test]
    fn rejects_truncated_mmm_offset_table() {
        assert!(Uk2MusicFile::from_bytes(Uk2MusicKind::Mmm, vec![0; 25]).is_err());
    }

    #[test]
    fn expands_mmd_delta_records_and_sysex_until_end_marker() {
        let records = decode_mmd_records(&[0x98, 0x00, 0x12, 0x34, 0xf7, 0xfe]).unwrap();
        assert_eq!(records.len(), 3);
        assert_eq!(records[0].bytes, [0x98, 0, 0, 0]);
        assert_eq!(records[1].bytes, [0x98, 0, 0x00, 0x12]);
        assert_eq!(records[2].bytes, [0x98, 0, 0x34, 0xf7]);
    }

    #[test]
    fn rejects_truncated_mmd_records_and_missing_end_marker() {
        assert!(decode_mmd_records(&[1, 2, 3]).is_err());
        assert!(decode_mmd_records(&[0x81]).is_err());
        assert!(decode_mmd_records(&[1, 2, 3, 4]).is_err());
    }
}
