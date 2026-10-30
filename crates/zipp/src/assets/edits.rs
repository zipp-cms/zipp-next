//! Edits of an image, kept next to the original instead of changing it:
//! the studio's image editor writes them, every transform starts from the
//! edited image, and resetting brings the original back. In this order:
//! rotate, flip, crop, resize, a canvas around it. A crop may reach past
//! the image and a canvas be bigger than it: that area is the background.

use image::imageops::FilterType;
use image::{DynamicImage, Rgba, RgbaImage};

use super::*;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edits {
	/// clockwise: 0, 90, 180 or 270
	#[serde(default)]
	pub rotate: u32,
	#[serde(default)]
	pub flip_h: bool,
	#[serde(default)]
	pub flip_v: bool,
	/// a part of the rotated image, 0 to 1 of its width and height, at
	/// most one image beyond each side
	pub crop: Option<Crop>,
	/// a new size, the height follows the width without one
	pub resize: Option<Resize>,
	/// a bigger area the image sits in the middle of
	pub canvas: Option<Canvas>,
	/// `#rrggbb` or `transparent` around the image, transparent without
	/// one
	#[serde(default)]
	pub background: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Crop {
	pub x: f64,
	pub y: f64,
	pub w: f64,
	pub h: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Resize {
	pub width: u32,
	pub height: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Canvas {
	pub width: u32,
	pub height: u32,
}

/// The biggest side anything may get.
const MAX_SIDE: u32 = 10_000;

impl Edits {
	pub fn is_empty(&self) -> bool {
		*self == Self::default()
	}

	/// Whether the crop reaches past the image.
	fn outside(&self) -> bool {
		self.crop.is_some_and(|c| {
			c.x < 0.0 || c.y < 0.0 || c.x + c.w > 1.0 || c.y + c.h > 1.0
		})
	}

	/// Whether the result needs an alpha channel.
	pub fn transparent(&self) -> bool {
		self.background
			.as_deref()
			.is_none_or(|b| b == "transparent")
			&& (self.canvas.is_some() || self.outside())
	}

	fn background(&self) -> Rgba<u8> {
		match self.background.as_deref() {
			Some(c) if c != "transparent" => {
				color(c).unwrap_or(Rgba([0, 0, 0, 0]))
			}
			_ => Rgba([0, 0, 0, 0]),
		}
	}

	pub fn check(&self) -> std::result::Result<(), String> {
		if ![0, 90, 180, 270].contains(&self.rotate) {
			return Err("rotate is 0, 90, 180 or 270".into());
		}
		if let Some(c) = self.crop {
			// one image beyond each side at most
			let near = |v: f64| (-1.0 - 1e-9..=2.0 + 1e-9).contains(&v);
			if !(near(c.x)
				&& near(c.y) && c.w > 0.0
				&& c.h > 0.0 && near(c.x + c.w)
				&& near(c.y + c.h))
			{
				return Err("the crop is too far outside the image".into());
			}
		}
		let side = |v: u32| (1..=MAX_SIDE).contains(&v);
		if let Some(r) = self.resize
			&& !(side(r.width) && r.height.is_none_or(side))
		{
			return Err(format!("a size is 1 to {MAX_SIDE} pixels"));
		}
		if let Some(c) = &self.canvas {
			if !(side(c.width) && side(c.height)) {
				return Err(format!("a size is 1 to {MAX_SIDE} pixels"));
			}
			if u64::from(c.width) * u64::from(c.height) > MAX_PIXELS {
				return Err("the canvas is bigger than 40 megapixels".into());
			}
		}
		if let Some(b) = &self.background
			&& b != "transparent"
			&& color(b).is_none()
		{
			return Err("the background is #rrggbb or transparent".into());
		}
		Ok(())
	}

	/// The size of a `width` x `height` image after the edits.
	pub fn size(&self, width: u32, height: u32) -> (u32, u32) {
		let (mut w, mut h) = match self.rotate {
			90 | 270 => (height, width),
			_ => (width, height),
		};
		if let Some(c) = self.crop {
			w = ((w as f64 * c.w).round() as u32).max(1);
			h = ((h as f64 * c.h).round() as u32).max(1);
		}
		if let Some(r) = self.resize {
			let height = r.height.unwrap_or_else(|| {
				((h as f64 * r.width as f64 / w as f64).round() as u32).max(1)
			});
			(w, h) = (r.width, height);
		}
		if let Some(c) = &self.canvas {
			(w, h) = (c.width, c.height);
		}
		(w, h)
	}

	/// The edited image.
	pub fn apply(&self, img: DynamicImage) -> DynamicImage {
		let mut img = match self.rotate {
			90 => img.rotate90(),
			180 => img.rotate180(),
			270 => img.rotate270(),
			_ => img,
		};
		if self.flip_h {
			img = img.fliph();
		}
		if self.flip_v {
			img = img.flipv();
		}
		if let Some(c) = self.crop {
			let (w, h) = (img.width() as f64, img.height() as f64);
			let x = (c.x * w).round() as i64;
			let y = (c.y * h).round() as i64;
			let cw = ((c.w * w).round() as u32).max(1);
			let ch = ((c.h * h).round() as u32).max(1);
			img = if self.outside() {
				// the image on the background, where the crop has it
				let mut out = RgbaImage::from_pixel(cw, ch, self.background());
				image::imageops::overlay(&mut out, &img.to_rgba8(), -x, -y);
				DynamicImage::ImageRgba8(out)
			} else {
				let (x, y) = (x as u32, y as u32);
				let (cw, ch) =
					(cw.min(img.width() - x), ch.min(img.height() - y));
				img.crop_imm(x, y, cw.max(1), ch.max(1))
			};
		}
		if let Some(r) = self.resize {
			let (w, h) = (img.width(), img.height());
			let height = r.height.unwrap_or_else(|| {
				((h as f64 * r.width as f64 / w as f64).round() as u32).max(1)
			});
			img = img.resize_exact(r.width, height, FilterType::Lanczos3);
		}
		if let Some(c) = &self.canvas {
			img = on_canvas(img, c, self.background());
		}
		img
	}
}

/// `#rrggbb` as a color.
fn color(hex: &str) -> Option<Rgba<u8>> {
	let hex = hex.strip_prefix('#').filter(|h| h.len() == 6)?;
	let v = u32::from_str_radix(hex, 16).ok()?;
	Some(Rgba([(v >> 16) as u8, (v >> 8) as u8, v as u8, 255]))
}

/// The image in the middle of the canvas, made smaller if it doesn't fit.
fn on_canvas(img: DynamicImage, c: &Canvas, bg: Rgba<u8>) -> DynamicImage {
	let mut canvas = RgbaImage::from_pixel(c.width, c.height, bg);
	let img = if img.width() > c.width || img.height() > c.height {
		img.resize(c.width, c.height, FilterType::Lanczos3)
	} else {
		img
	};
	let x = (c.width - img.width()) / 2;
	let y = (c.height - img.height()) / 2;
	let img = img.to_rgba8();
	// blend, so transparent parts of the image show the background
	image::imageops::overlay(&mut canvas, &img, x.into(), y.into());
	DynamicImage::ImageRgba8(canvas)
}
