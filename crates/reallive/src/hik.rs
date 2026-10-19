//! HIK animated backgrounds (`bgrLoadHaikei` with a `.hik` file): layers
//! of image frames, optionally scrolling and clipped, that animate on
//! their own clock. The format is a flat list of tagged little-endian
//! integers (and length-prefixed strings); the tags follow rlvm's reading
//! of it.

use std::rc::Rc;

use anyhow::{Context, Result, bail};

use crate::image::Image;
use crate::surface::{Blend, Rect, Surface};

#[derive(Debug, Clone)]
pub struct Frame {
    pub opacity: i32,
    pub image: Rc<Image>,
    /// Pattern of the image, or -1 for the first.
    pub pattern: i32,
    pub length_ms: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Animation {
    /// Frames advance with time (otherwise the first is shown).
    pub multiframe: bool,
    /// At the end: 0 repeat, 3 continue with the layer's next animation.
    pub next_mode: i32,
    pub frames: Vec<Frame>,
    pub total_ms: i32,
}

#[derive(Debug, Clone, Default)]
pub struct Layer {
    pub offset: (i32, i32),
    pub scrolling: bool,
    pub scroll_from: (i32, i32),
    pub scroll_to: (i32, i32),
    /// Duration of one horizontal / vertical scroll cycle.
    pub scroll_ms: (i32, i32),
    pub use_clip: bool,
    pub clip: Option<Rect>,
    pub animations: Vec<Animation>,
}

#[derive(Debug, Clone, Default)]
pub struct HikScript {
    pub size: (i32, i32),
    pub layers: Vec<Layer>,
}

struct Reader<'a> {
    data: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn i32(&mut self) -> Result<i32> {
        let bytes = self
            .data
            .get(self.at..self.at + 4)
            .context("HIK: truncated")?;
        self.at += 4;
        Ok(i32::from_le_bytes(bytes.try_into()?))
    }

    fn string(&mut self) -> Result<String> {
        let length = self.i32()?.max(0) as usize;
        let bytes = self
            .data
            .get(self.at..self.at + length)
            .context("HIK: truncated string")?;
        self.at += length;
        let text = bytes.split(|&b| b == 0).next().unwrap_or(&[]);
        Ok(String::from_utf8_lossy(text).into_owned())
    }

    fn skip(&mut self, words: usize) -> Result<()> {
        for _ in 0..words {
            self.i32()?;
        }
        Ok(())
    }
}

impl HikScript {
    /// Parses a HIK file; `load` fetches the images its frames name.
    pub fn parse(data: &[u8], mut load: impl FnMut(&str) -> Result<Rc<Image>>) -> Result<Self> {
        let mut r = Reader { data, at: 0 };
        if r.i32()? != 10000 || r.i32()? != 10000 {
            bail!("HIK: bad magic");
        }
        let mut script = Self::default();
        fn layer(script: &mut HikScript) -> Result<&mut Layer> {
            script.layers.last_mut().context("HIK: data before a layer")
        }
        fn animation(script: &mut HikScript) -> Result<&mut Animation> {
            layer(script)?
                .animations
                .last_mut()
                .context("HIK: data before an animation")
        }
        fn frame(script: &mut HikScript) -> Result<&mut Frame> {
            animation(script)?
                .frames
                .last_mut()
                .context("HIK: data before a frame")
        }
        while r.at < data.len() {
            let tag = r.i32()?;
            match tag {
                10100..=10102 | 20000 | 21000 | 21003 | 21100 | 21203 | 30000 | 40000 => {
                    r.skip(1)?
                }
                10103 => script.size = (r.i32()?, r.i32()?),
                20001 => {
                    r.skip(1)?;
                    script.layers.push(Layer::default());
                }
                20100 => {
                    r.string()?;
                }
                20101 => layer(&mut script)?.offset = (r.i32()?, r.i32()?),
                21001 | 21101 => r.skip(4)?,
                21002 => r.skip(5)?,
                21200 => layer(&mut script)?.scrolling = r.i32()? != 0,
                21201 => {
                    let from = (r.i32()?, r.i32()?);
                    let to = (r.i32()?, r.i32()?);
                    let l = layer(&mut script)?;
                    l.scroll_from = from;
                    l.scroll_to = to;
                }
                21202 => layer(&mut script)?.scroll_ms = (r.i32()?, r.i32()?),
                21301 => layer(&mut script)?.use_clip = r.i32()? != 0,
                21300 => {
                    let (x1, y1, x2, y2) = (r.i32()?, r.i32()?, r.i32()?, r.i32()?);
                    layer(&mut script)?.clip = Some(Rect::from_corners(x1, y1, x2, y2));
                }
                30001 => {
                    r.skip(1)?;
                    layer(&mut script)?.animations.push(Animation::default());
                }
                30100 => animation(&mut script)?.multiframe = r.i32()? != 0,
                30101 => animation(&mut script)?.next_mode = r.i32()?,
                30102 => r.skip(1)?,
                40101 => {
                    r.skip(31)?;
                    animation(&mut script)?.frames.push(Frame {
                        opacity: 255,
                        image: Rc::new(Image::new(1, 1)),
                        pattern: -1,
                        length_ms: 0,
                    });
                }
                40102 => frame(&mut script)?.opacity = r.i32()?,
                40103 => r.skip(2)?,
                40100 => {
                    let name = r.string()?;
                    let image = load(&name).with_context(|| format!("HIK frame image {name}"))?;
                    let (pattern, length) = (r.i32()?, r.i32()?);
                    let f = frame(&mut script)?;
                    f.image = image;
                    f.pattern = pattern;
                    f.length_ms = length;
                }
                other => bail!("HIK: unknown tag {other}"),
            }
        }
        for layer in &mut script.layers {
            if !layer.use_clip {
                layer.clip = None;
            }
            for animation in &mut layer.animations {
                animation.total_ms = animation.frames.iter().map(|f| f.length_ms.max(0)).sum();
            }
        }
        // Records come last layer first.
        script.layers.reverse();
        Ok(script)
    }
}

/// A HIK script being shown.
#[derive(Debug, Clone)]
pub struct HikRenderer {
    /// The file it came from (saved games re-load it).
    pub name: String,
    pub script: Rc<HikScript>,
    pub created: u64,
    /// Per layer: (animation, when it started).
    layers: Vec<(usize, u64)>,
    /// `HAIKEI_SET_POS*`: shifts what each frame shows.
    pub offset: (i32, i32),
}

impl HikRenderer {
    pub fn new(name: &str, script: Rc<HikScript>, now: u64) -> Self {
        let layers = vec![(0, now); script.layers.len()];
        Self {
            name: name.to_owned(),
            script,
            created: now,
            layers,
            offset: (0, 0),
        }
    }

    /// `HAIKEI_NEXT`: every layer moves on to its next animation.
    pub fn next_animation(&mut self, now: u64) {
        for (index, state) in self.layers.iter_mut().enumerate() {
            let count = self.script.layers[index].animations.len().max(1);
            *state = ((state.0 + 1) % count, now);
        }
    }

    pub fn render(&mut self, frame: &mut Surface, now: u64) {
        let since = now.saturating_sub(self.created) as i64;
        for (index, layer) in self.script.layers.iter().enumerate() {
            if layer.animations.is_empty() {
                continue;
            }
            let mut dest = layer.offset;
            if layer.scrolling {
                dest.0 += layer.scroll_from.0;
                dest.1 += layer.scroll_from.1;
                let (xt, yt) = layer.scroll_ms;
                if xt > 0 {
                    let p = (since % i64::from(xt)) as f64 / f64::from(xt);
                    dest.0 += (f64::from(layer.scroll_to.0 - layer.scroll_from.0) * p) as i32;
                }
                if yt > 0 {
                    let p = (since % i64::from(yt)) as f64 / f64::from(yt);
                    dest.1 += (f64::from(layer.scroll_to.1 - layer.scroll_from.1) * p) as i32;
                }
            }
            let (ref mut number, ref mut started) = self.layers[index];
            *number = (*number).min(layer.animations.len() - 1);
            let mut animation = &layer.animations[*number];
            let mut chosen = 0;
            if animation.multiframe && animation.total_ms > 0 {
                let mut elapsed = now.saturating_sub(*started) as i64;
                let mut advanced = false;
                while elapsed > i64::from(animation.total_ms) {
                    if animation.next_mode == 3 {
                        *number = (*number + 1) % layer.animations.len();
                    }
                    elapsed -= i64::from(animation.total_ms);
                    animation = &layer.animations[*number];
                    advanced = true;
                    if animation.total_ms <= 0 {
                        break;
                    }
                }
                if advanced {
                    *started = now.saturating_sub(elapsed.max(0) as u64);
                }
                for (i, f) in animation.frames.iter().enumerate() {
                    chosen = i;
                    elapsed -= i64::from(f.length_ms.max(0));
                    if elapsed < 0 {
                        break;
                    }
                }
            }
            let Some(f) = animation.frames.get(chosen) else {
                continue;
            };
            let region = f
                .image
                .regions
                .get(f.pattern.max(0) as usize)
                .copied()
                .unwrap_or(crate::image::Region::full(
                    f.image.surface.width,
                    f.image.surface.height,
                ));
            let mut src = Rect::from_corners(
                region.x1 + self.offset.0,
                region.y1 + self.offset.1,
                region.x2 + 1 + self.offset.0,
                region.y2 + 1 + self.offset.1,
            );
            let mut at = dest;
            if let Some(clip) = layer.clip {
                // Clip the destination, moving the source with it.
                let d = Rect::new(at.0, at.1, src.w, src.h).intersect(&clip);
                src = Rect::new(src.x + d.x - at.0, src.y + d.y - at.1, d.w, d.h);
                at = (d.x, d.y);
            }
            if src.w > 0 && src.h > 0 {
                let opacity = f.opacity.clamp(0, 255) as u8;
                frame.blit(
                    &f.image.surface,
                    src,
                    at.0,
                    at.1,
                    opacity,
                    Blend::Mask,
                    None,
                );
            }
        }
    }
}
