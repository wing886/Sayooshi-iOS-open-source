use std::io::{Cursor, Read, Seek, SeekFrom};
use std::sync::Arc;

use anyhow::{Context, Result, anyhow, bail};
use lewton::audio::{PreviousWindowRight, read_audio_packet_generic};
use lewton::header::{
    CommentHeader, IdentHeader, SetupHeader, read_header_comment, read_header_ident,
    read_header_setup,
};
use lewton::samples::InterleavedSamples;
use ogg::reading::PacketReader;
use theora_rs::{HeaderParser, OggPacket, PixelFmt};
pub use theora_rs::{ImgPlaneRef, YCbCrRef};

pub const TH_PF_420: i32 = 0;
pub const TH_PF_422: i32 = 2;
pub const TH_PF_444: i32 = 3;

#[derive(Debug, Clone, Copy)]
pub struct VideoInfo {
    /// Theora picture-region width reported by the bitstream.
    pub width: i32,
    /// Theora picture-region height reported by the bitstream.
    pub height: i32,
    /// Full decoded luma-plane width returned by `th_decode_ycbcr_out`.
    /// Siglus OMV reads from this uncropped plane and applies the OMV header's
    /// own display size instead of Theora's picture rectangle.
    pub frame_width: i32,
    /// Full decoded luma-plane height returned by `th_decode_ycbcr_out`.
    pub frame_height: i32,
    pub pic_x: i32,
    pub pic_y: i32,
    pub fps: f64,
    pub fmt: i32,
}

#[derive(Clone)]
struct SharedBytes(Arc<Vec<u8>>);

impl AsRef<[u8]> for SharedBytes {
    fn as_ref(&self) -> &[u8] {
        self.0.as_slice()
    }
}

pub struct TheoraFile {
    info: VideoInfo,
    ogg_data: SharedBytes,
    video_stream: Option<TheoraVideoStream<Cursor<SharedBytes>>>,
    audio_channels: i32,
    audio_sample_rate: i32,
    has_audio_stream: bool,
    audio_stream: Option<VorbisStream>,
}

struct VorbisStream {
    reader: PacketReader<Cursor<SharedBytes>>,
    serial: u32,
    ident: IdentHeader,
    setup: SetupHeader,
    previous_window: PreviousWindowRight,
    pending_samples: Vec<f32>,
    pending_offset: usize,
}

/// Incremental Theora video decoder over an Ogg reader.
///
/// This keeps the Ogg packet reader and Theora decoder alive and decodes only
/// the next requested video packet. It deliberately ignores non-Theora logical
/// streams, so an OMV playback worker does not have to read the complete file or
/// decode its full video before the first frame can be displayed.
pub struct TheoraVideoStream<R: Read + Seek> {
    reader: PacketReader<R>,
    video_serial: u32,
    decoder: theora_rs::DecoderContext,
    info: VideoInfo,
    /// Number of Theora header packets consumed before the first video packet.
    header_packet_count: i64,
    packet_no: i64,
}

impl<R: Read + Seek> TheoraVideoStream<R> {
    pub fn open(reader: R) -> Result<Self> {
        let mut reader = PacketReader::new(reader);
        let mut parser = HeaderParser::new();
        let mut video_serial = None;
        let mut packet_no = 0i64;

        while let Some(pkt) = reader.read_packet().context("ogg packet read")? {
            let serial = pkt.stream_serial();
            if video_serial.is_none()
                && pkt.first_in_stream()
                && pkt.data.len() >= 7
                && pkt.data[0] == 0x80
                && &pkt.data[1..7] == b"theora"
            {
                video_serial = Some(serial);
            }
            if Some(serial) != video_serial {
                continue;
            }

            let b_o_s = pkt.first_in_stream();
            let e_o_s = pkt.last_in_stream();
            let granulepos = pkt.absgp_page() as i64;
            let data = pkt.data;
            let op = OggPacket {
                packet: data,
                b_o_s,
                e_o_s,
                granulepos,
                packetno: packet_no,
            };
            packet_no = packet_no.saturating_add(1);
            let _ = parser.push(&op)?;
            if parser.is_ready() {
                let decoder = parser.decoder()?;
                let info = video_info_from_theora_info(&parser.info);
                return Ok(Self {
                    reader,
                    video_serial: serial,
                    decoder,
                    info,
                    header_packet_count: packet_no,
                    packet_no,
                });
            }
        }

        bail!("stream ended before all Theora headers were parsed")
    }

    pub fn info(&self) -> VideoInfo {
        self.info
    }

    pub fn read_video_frame(&mut self) -> Result<Option<Vec<u8>>> {
        self.read_video_frame_with(|ycbcr| pack_theorafile_frame(ycbcr))?
            .transpose()
    }

    /// Decodes the next frame and passes its planes, as the decoder holds
    /// them, to `f` (no repacked copy).
    pub fn read_video_frame_with<T>(
        &mut self,
        f: impl FnOnce(&YCbCrRef<'_>) -> T,
    ) -> Result<Option<T>> {
        while let Some(pkt) = self.reader.read_packet().context("ogg packet read")? {
            if pkt.stream_serial() != self.video_serial {
                continue;
            }
            let b_o_s = pkt.first_in_stream();
            let e_o_s = pkt.last_in_stream();
            let granulepos = pkt.absgp_page() as i64;
            let data = pkt.data;
            let op = OggPacket {
                packet: data,
                b_o_s,
                e_o_s,
                granulepos,
                packetno: self.packet_no,
            };
            self.packet_no = self.packet_no.saturating_add(1);
            // Empty Ogg packets advance granule state but do not produce a new
            // decoded frame. Reset this edge-triggered flag before each packet
            // so the previous frame is never emitted twice.
            self.decoder.frame_available = false;
            theora_rs::th_decode_packetin(&mut self.decoder, &op)?;
            if self.decoder.has_decoded_frame() {
                let ycbcr = theora_rs::th_decode_ycbcr_ref(&self.decoder)?;
                return Ok(Some(f(&ycbcr)));
            }
        }
        Ok(None)
    }

    /// Seek to an OMV packet using the original page/key-frame index.
    ///
    /// `file_offset` points to the indexed `seek_page_no`, which can precede
    /// the key-frame page when an Ogg packet spans pages. `first_packet_no` is
    /// the key-frame page's `top_packet_no`: this is where tona3 starts its
    /// decode packet counter after feeding/draining the back-pages.  In
    /// particular, the seek page itself is allowed to have `top_packet_no=-1`.
    /// Packets completed before `key_page_file_offset` must be drained without
    /// advancing that counter. The retained decoder context decodes the key frame through
    /// `target_packet_no`.
    pub fn seek_to_indexed_frame(
        &mut self,
        file_offset: u64,
        key_page_file_offset: u64,
        first_packet_no: usize,
        key_frame_packet_no: usize,
        target_packet_no: usize,
    ) -> Result<Option<Vec<u8>>> {
        if key_page_file_offset < file_offset {
            bail!("Theora key-frame page precedes seek page");
        }
        if first_packet_no > key_frame_packet_no {
            bail!(
                "Theora seek packet {} follows key packet {}",
                first_packet_no,
                key_frame_packet_no
            );
        }
        if target_packet_no < key_frame_packet_no {
            bail!(
                "Theora target packet {} precedes key packet {}",
                target_packet_no,
                key_frame_packet_no
            );
        }

        self.reader
            .seek_bytes(SeekFrom::Start(file_offset))
            .context("seek indexed Ogg page")?;

        let mut data_packet_no = first_packet_no;
        let mut reached_key_page = false;
        while let Some(pkt) = self
            .reader
            .read_packet()
            .context("ogg packet read after indexed seek")?
        {
            if !reached_key_page {
                // PacketReader drains all completed packets from a page before
                // reading the next page. Its reader position is the end of the
                // page completing this packet, including for spanning packets.
                // Back-pages can complete unrelated packets as well as start
                // the key packet; tona3 discards those before numbering packets
                // from the key page's top_packet_no.
                if self.reader.get_mut().stream_position()? <= key_page_file_offset {
                    continue;
                }
                reached_key_page = true;
            }
            if pkt.stream_serial() != self.video_serial {
                continue;
            }
            if theora_rs::th_packet_isheader(theora_rs::Packet::new(&pkt.data)) {
                continue;
            }

            if data_packet_no < key_frame_packet_no {
                data_packet_no = data_packet_no.saturating_add(1);
                continue;
            }
            if data_packet_no > target_packet_no {
                bail!(
                    "indexed Theora seek passed target packet {}",
                    target_packet_no
                );
            }
            if data_packet_no == key_frame_packet_no
                && theora_rs::th_packet_iskeyframe(theora_rs::Packet::new(&pkt.data)) != 1
            {
                bail!(
                    "indexed Theora packet {} is not a key frame",
                    key_frame_packet_no
                );
            }

            let b_o_s = pkt.first_in_stream();
            let e_o_s = pkt.last_in_stream();
            let granulepos = pkt.absgp_page() as i64;
            let data = pkt.data;
            let packetno = self
                .header_packet_count
                .saturating_add(data_packet_no as i64);
            let op = OggPacket {
                packet: data,
                b_o_s,
                e_o_s,
                granulepos,
                packetno,
            };
            self.decoder.frame_available = false;
            theora_rs::th_decode_packetin(&mut self.decoder, &op)?;
            self.packet_no = packetno.saturating_add(1);

            if data_packet_no == target_packet_no {
                if self.decoder.has_decoded_frame() {
                    let ycbcr = theora_rs::th_decode_ycbcr_ref(&self.decoder)?;
                    return Ok(Some(pack_theorafile_frame(&ycbcr)?));
                }
                return Ok(None);
            }
            data_packet_no = data_packet_no.saturating_add(1);
        }

        bail!(
            "indexed Theora seek ended before target packet {}",
            target_packet_no
        )
    }
}

fn video_info_from_theora_info(info: &theora_rs::Info) -> VideoInfo {
    let width = info.pic_width.max(1) as i32;
    let height = info.pic_height.max(1) as i32;
    let frame_width = info.frame_width.max(1) as i32;
    let frame_height = info.frame_height.max(1) as i32;
    let fps = if info.fps_denominator != 0 {
        info.fps_numerator as f64 / info.fps_denominator as f64
    } else {
        0.0
    };
    let fmt = match info.pixel_fmt {
        PixelFmt::Pf420 | PixelFmt::Reserved => TH_PF_420,
        PixelFmt::Pf422 => TH_PF_422,
        PixelFmt::Pf444 => TH_PF_444,
    };
    VideoInfo {
        width,
        height,
        frame_width,
        frame_height,
        pic_x: info.pic_x as i32,
        pic_y: info.pic_y as i32,
        fps,
        fmt,
    }
}

pub fn decode_first_video_frame_from_memory(data: Vec<u8>) -> Result<(VideoInfo, Vec<u8>)> {
    // A preview needs one frame. Parsing every Ogg packet first retains the
    // entire compressed movie, including its audio stream, before decoding.
    let mut stream =
        TheoraVideoStream::open(Cursor::new(data)).context("open Theora preview stream")?;
    let info = stream.info();
    let frame = stream
        .read_video_frame()
        .context("decode first theora frame")?
        .ok_or_else(|| anyhow!("no decoded Theora frame in stream"))?;
    Ok((info, frame))
}

impl TheoraFile {
    pub fn open_from_memory(data: Vec<u8>) -> Result<Self> {
        let ogg_data = SharedBytes(Arc::new(data));
        let video_stream =
            TheoraVideoStream::open(Cursor::new(ogg_data.clone())).context("open Theora stream")?;
        let info = video_stream.info();
        let audio_stream = VorbisStream::open(Cursor::new(ogg_data.clone()))?;
        let (has_audio_stream, audio_channels, audio_sample_rate) = audio_stream
            .as_ref()
            .map(|stream| {
                (
                    true,
                    stream.ident.audio_channels as i32,
                    stream.ident.audio_sample_rate as i32,
                )
            })
            .unwrap_or((false, 0, 0));

        Ok(Self {
            info,
            ogg_data,
            video_stream: Some(video_stream),
            audio_channels,
            audio_sample_rate,
            has_audio_stream,
            audio_stream,
        })
    }

    pub fn info(&self) -> VideoInfo {
        self.info
    }

    pub fn has_audio(&self) -> bool {
        self.has_audio_stream
    }

    pub fn audio_info(&self) -> Option<(i32, i32)> {
        if !self.has_audio_stream || self.audio_channels <= 0 || self.audio_sample_rate <= 0 {
            return None;
        }
        Some((self.audio_channels, self.audio_sample_rate))
    }

    pub fn reset(&mut self) {
        // Reopen on the next read so reset remains infallible. The stream was
        // already validated when this file was opened.
        self.video_stream = None;
        self.audio_stream = None;
    }

    pub fn read_video_frame(&mut self, out: &mut [u8]) -> Result<bool> {
        if self.video_stream.is_none() {
            self.video_stream = Some(
                TheoraVideoStream::open(Cursor::new(self.ogg_data.clone()))
                    .context("reopen Theora stream after reset")?,
            );
        }
        let stream = self.video_stream.as_mut().expect("stream reopened above");
        match stream.read_video_frame_with(|planes| copy_decoded_planes_into(out, planes))? {
            Some(result) => result.map(|()| true),
            None => Ok(false),
        }
    }

    pub fn read_audio_samples(&mut self, out: &mut [f32]) -> Result<usize> {
        if out.is_empty() || !self.has_audio_stream {
            return Ok(0);
        }
        if self.audio_stream.is_none() {
            self.audio_stream = VorbisStream::open(Cursor::new(self.ogg_data.clone()))?;
        }
        self.audio_stream
            .as_mut()
            .expect("audio stream reopened above")
            .read_samples(out)
    }
}

impl VorbisStream {
    fn open(source: Cursor<SharedBytes>) -> Result<Option<Self>> {
        let mut reader = PacketReader::new(source);
        let mut audio_serial = None;
        let mut headers = Vec::with_capacity(3);
        while let Some(packet) = reader.read_packet().context("ogg packet read")? {
            let serial = packet.stream_serial();
            if audio_serial.is_none()
                && packet.first_in_stream()
                && packet.data.len() >= 7
                && packet.data[0] == 0x01
                && &packet.data[1..7] == b"vorbis"
            {
                audio_serial = Some(serial);
            }
            if Some(serial) != audio_serial {
                continue;
            }
            headers.push(packet.data);
            if headers.len() == 3 {
                let ident = read_header_ident(&headers[0]).context("read vorbis ident header")?;
                let _comment: CommentHeader =
                    read_header_comment(&headers[1]).context("read vorbis comment header")?;
                let setup = read_header_setup(
                    &headers[2],
                    ident.audio_channels,
                    (ident.blocksize_0, ident.blocksize_1),
                )
                .context("read vorbis setup header")?;
                return Ok(Some(Self {
                    reader,
                    serial,
                    ident,
                    setup,
                    previous_window: PreviousWindowRight::new(),
                    pending_samples: Vec::new(),
                    pending_offset: 0,
                }));
            }
        }
        if audio_serial.is_some() {
            bail!("vorbis stream does not contain enough header packets");
        }
        Ok(None)
    }

    fn read_samples(&mut self, out: &mut [f32]) -> Result<usize> {
        let mut written = 0;
        while written < out.len() {
            if self.pending_offset < self.pending_samples.len() {
                let available = self.pending_samples.len() - self.pending_offset;
                let count = available.min(out.len() - written);
                out[written..written + count].copy_from_slice(
                    &self.pending_samples[self.pending_offset..self.pending_offset + count],
                );
                self.pending_offset += count;
                written += count;
                continue;
            }
            let Some(packet) = self.reader.read_packet().context("ogg packet read")? else {
                break;
            };
            if packet.stream_serial() != self.serial {
                continue;
            }
            let decoded: InterleavedSamples<f32> = read_audio_packet_generic(
                &self.ident,
                &self.setup,
                &packet.data,
                &mut self.previous_window,
            )
            .context("decode vorbis audio packet")?;
            self.pending_samples = decoded.samples;
            self.pending_offset = 0;
        }
        Ok(written)
    }
}

fn pack_theorafile_frame(ycbcr: &theora_rs::YCbCrRef<'_>) -> Result<Vec<u8>> {
    // Match tona3's C_omv_player_impl::video_write() input semantics. The
    // original code reads directly from th_decode_ycbcr_out() using each
    // plane's stride and the OMV header's own display width/height. It does
    // not apply Theora pic_x/pic_y or pic_width/pic_height here.
    //
    // We still make the worker message self-contained by repacking each full
    // decoded plane tightly. Crucially, this preserves the coded-frame columns
    // and rows (including RGBA alpha rows below the visible OMV image) while
    // discarding only decoder stride padding outside plane.width.
    let capacity = ycbcr.iter().fold(0usize, |acc, plane| {
        acc.saturating_add(
            usize::try_from(plane.width.max(0))
                .unwrap_or(0)
                .saturating_mul(usize::try_from(plane.height.max(0)).unwrap_or(0)),
        )
    });
    let mut out = Vec::with_capacity(capacity);
    for plane in ycbcr {
        copy_decoded_plane_tight(&mut out, plane)?;
    }
    Ok(out)
}

fn copy_decoded_planes_into(dst: &mut [u8], planes: &YCbCrRef<'_>) -> Result<()> {
    let mut written = 0usize;
    for plane in planes {
        let width = usize::try_from(plane.width)
            .map_err(|_| anyhow!("negative decoded plane width: {}", plane.width))?;
        let height = usize::try_from(plane.height)
            .map_err(|_| anyhow!("negative decoded plane height: {}", plane.height))?;
        let len = width
            .checked_mul(height)
            .ok_or_else(|| anyhow!("decoded plane size overflow"))?;
        let end = written
            .checked_add(len)
            .ok_or_else(|| anyhow!("decoded frame size overflow"))?;
        let dst_len = dst.len();
        let output = dst.get_mut(written..end).ok_or_else(|| {
            anyhow!(
                "video output buffer too small: need {} bytes, got {} bytes",
                end,
                dst_len
            )
        })?;
        copy_decoded_plane_into(output, plane, width, height)?;
        written = end;
    }
    Ok(())
}

fn copy_decoded_plane_into(
    dst: &mut [u8],
    plane: &theora_rs::ImgPlaneRef<'_>,
    width: usize,
    height: usize,
) -> Result<()> {
    if width == 0 || height == 0 {
        return Ok(());
    }
    let base = isize::try_from(plane.data_offset)
        .map_err(|_| anyhow!("decoded plane offset does not fit isize"))?;
    let stride = plane.stride as isize;
    if stride == width as isize {
        let start =
            usize::try_from(base).map_err(|_| anyhow!("decoded plane starts before buffer"))?;
        let end = start
            .checked_add(dst.len())
            .ok_or_else(|| anyhow!("decoded plane end overflow"))?;
        let pixels = plane
            .data
            .get(start..end)
            .ok_or_else(|| anyhow!("decoded plane copy out of range"))?;
        dst.copy_from_slice(pixels);
        return Ok(());
    }
    for row in 0..height {
        let row_offset = (row as isize)
            .checked_mul(stride)
            .and_then(|offset| base.checked_add(offset))
            .ok_or_else(|| anyhow!("decoded plane row offset overflow"))?;
        let start = usize::try_from(row_offset)
            .map_err(|_| anyhow!("decoded plane row starts before buffer"))?;
        let end = start
            .checked_add(width)
            .ok_or_else(|| anyhow!("decoded plane row end overflow"))?;
        let src = plane
            .data
            .get(start..end)
            .ok_or_else(|| anyhow!("decoded plane copy out of range"))?;
        dst[row * width..(row + 1) * width].copy_from_slice(src);
    }
    Ok(())
}

fn copy_decoded_plane_tight(dst: &mut Vec<u8>, plane: &theora_rs::ImgPlaneRef<'_>) -> Result<()> {
    let width = usize::try_from(plane.width)
        .map_err(|_| anyhow!("negative decoded plane width: {}", plane.width))?;
    let height = usize::try_from(plane.height)
        .map_err(|_| anyhow!("negative decoded plane height: {}", plane.height))?;
    if width == 0 || height == 0 {
        return Ok(());
    }

    let base = isize::try_from(plane.data_offset)
        .map_err(|_| anyhow!("decoded plane offset does not fit isize"))?;
    let stride = plane.stride as isize;
    let plane_len = width
        .checked_mul(height)
        .ok_or_else(|| anyhow!("decoded plane size overflow"))?;
    // Coded planes without row padding are common. Copying one contiguous
    // span avoids a slice check and memcpy call for every decoded row.
    if stride == width as isize {
        let start =
            usize::try_from(base).map_err(|_| anyhow!("decoded plane starts before buffer"))?;
        let end = start
            .checked_add(plane_len)
            .ok_or_else(|| anyhow!("decoded plane end overflow"))?;
        let pixels = plane
            .data
            .get(start..end)
            .ok_or_else(|| anyhow!("decoded plane copy out of range"))?;
        dst.extend_from_slice(pixels);
        return Ok(());
    }
    dst.reserve(plane_len);

    for row in 0..height {
        let row_i = isize::try_from(row).map_err(|_| anyhow!("decoded plane row overflow"))?;
        let start = base
            .checked_add(
                stride
                    .checked_mul(row_i)
                    .ok_or_else(|| anyhow!("decoded plane stride overflow"))?,
            )
            .ok_or_else(|| anyhow!("decoded plane row offset overflow"))?;
        if start < 0 {
            bail!(
                "decoded plane row starts before buffer: row={} start={} stride={} offset={}",
                row,
                start,
                plane.stride,
                plane.data_offset
            );
        }
        let start = usize::try_from(start).map_err(|_| anyhow!("decoded plane row overflow"))?;
        let end = start
            .checked_add(width)
            .ok_or_else(|| anyhow!("decoded plane row end overflow"))?;
        if end > plane.data.len() {
            bail!(
                "decoded plane copy out of range: row={} start={} end={} len={} stride={} width={} height={} offset={}",
                row,
                start,
                end,
                plane.data.len(),
                plane.stride,
                width,
                height,
                plane.data_offset
            );
        }
        dst.extend_from_slice(&plane.data[start..end]);
    }
    Ok(())
}

#[cfg(test)]
mod omv_plane_parity_tests {
    use super::{copy_decoded_plane_tight, pack_theorafile_frame};
    use theora_rs::ImgPlane;

    #[test]
    fn full_decoded_plane_keeps_coded_columns_and_rows() {
        // Two bytes of decoder padding follow every 4-pixel coded row. The
        // OMV path must keep all four coded pixels, not crop a Theora picture
        // rectangle out of the plane.
        let plane = ImgPlane {
            width: 4,
            height: 3,
            stride: 6,
            data: vec![
                1, 2, 3, 4, 90, 91, 5, 6, 7, 8, 92, 93, 9, 10, 11, 12, 94, 95,
            ],
            data_offset: 0,
        };
        let mut out = Vec::new();
        copy_decoded_plane_tight(&mut out, &plane.as_ref()).unwrap();
        assert_eq!(out, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }

    #[test]
    fn full_decoded_plane_supports_negative_stride() {
        let plane = ImgPlane {
            width: 3,
            height: 2,
            stride: -4,
            data: vec![4, 5, 6, 90, 1, 2, 3, 91],
            data_offset: 4,
        };
        let mut out = Vec::new();
        copy_decoded_plane_tight(&mut out, &plane.as_ref()).unwrap();
        assert_eq!(out, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn packed_frame_concatenates_full_planes_without_picture_crop() {
        let mk = |base: u8| ImgPlane {
            width: 2,
            height: 2,
            stride: 3,
            data: vec![base, base + 1, 0, base + 2, base + 3, 0],
            data_offset: 0,
        };
        let planes = [mk(1), mk(10), mk(20)];
        let packed =
            pack_theorafile_frame(&[planes[0].as_ref(), planes[1].as_ref(), planes[2].as_ref()])
                .unwrap();
        assert_eq!(packed, vec![1, 2, 3, 4, 10, 11, 12, 13, 20, 21, 22, 23]);
    }
}
