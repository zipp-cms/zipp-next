use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, RgbImage, Rgba, RgbaImage};

use super::*;

/// The size of a `width` x `height` image after a transform, never bigger
/// than the image.
pub fn transform_size(width: u32, height: u32, t: &Transform) -> (u32, u32) {
	let (w, h) = (width as f64, height as f64);
	let scale_w = t.width.map_or(f64::MAX, |tw| tw as f64 / w);
	let scale_h = t.height.map_or(f64::MAX, |th| th as f64 / h);

	if t.crop {
		let (tw, th) = (t.width.unwrap() as f64, t.height.unwrap() as f64);
		let s = (w / tw).min(h / th).min(1.0);
		return ((tw * s).round() as u32, (th * s).round() as u32);
	}

	let s = scale_w.min(scale_h).min(1.0);
	(
		((w * s).round() as u32).max(1),
		((h * s).round() as u32).max(1),
	)
}

/// The part of the image a crop to `out_w` x `out_h` keeps: the biggest
/// rect of that ratio, as close to the focal point as it fits.
pub(crate) fn crop_rect(
	width: u32,
	height: u32,
	out_w: u32,
	out_h: u32,
	focal: (f64, f64),
) -> (u32, u32, u32, u32) {
	let ratio = out_w as f64 / out_h as f64;
	let (cw, ch) = if width as f64 / height as f64 > ratio {
		((height as f64 * ratio).round() as u32, height)
	} else {
		(width, (width as f64 / ratio).round() as u32)
	};
	let x = (focal.0 * width as f64 - cw as f64 / 2.0)
		.clamp(0.0, (width - cw) as f64);
	let y = (focal.1 * height as f64 - ch as f64 / 2.0)
		.clamp(0.0, (height - ch) as f64);
	(x.round() as u32, y.round() as u32, cw, ch)
}

/// Writes the transform of an image to `out` as a `mime`, made of the
/// image with its edits.
pub(super) fn generate(
	asset: &Asset,
	original: &FsPath,
	t: &Transform,
	mime: &str,
	out: &FsPath,
) -> Result<()> {
	let internal = |e: &dyn std::fmt::Display| Error::Internal(e.to_string());
	// by its content, a name may have no or another extension
	let mut reader = image::ImageReader::open(original)
		.and_then(|r| r.with_guessed_format())
		.map_err(|e| internal(&e))?;
	reader.limits(decode_limits());
	let img = reader.decode().map_err(|e| internal(&e))?;
	let img = match &asset.edits {
		// edits saved before a limit was lower must not take all memory
		Some(e) => {
			e.check_size(img.width(), img.height())
				.map_err(|e| internal(&e))?;
			e.apply(img)
		}
		None => img,
	};
	let format = ImageFormat::from_mime_type(mime)
		.ok_or_else(|| internal(&"an image of an unknown type"))?;
	let (w, h) = (img.width(), img.height());
	let (out_w, out_h) = transform_size(w, h, t);

	let img = if t.crop {
		let focal = (asset.focal_x, asset.focal_y);
		let (x, y, cw, ch) = crop_rect(w, h, out_w, out_h, focal);
		img.crop_imm(x, y, cw, ch).resize_exact(
			out_w,
			out_h,
			FilterType::Lanczos3,
		)
	} else {
		img.resize_exact(out_w, out_h, FilterType::Lanczos3)
	};

	// write next to it and rename, two requests may generate it at once
	fs::create_dir_all(out.parent().unwrap()).map_err(|e| internal(&e))?;
	let tmp = out.with_extension(format!("tmp{}", rand::random::<u32>()));
	match format {
		ImageFormat::Jpeg => {
			let file = fs::File::create(&tmp).map_err(|e| internal(&e))?;
			let encoder = JpegEncoder::new_with_quality(file, 82);
			on_white(img)
				.write_with_encoder(encoder)
				.map_err(|e| internal(&e))?;
		}
		// image's own encoder is lossless only, too big for photos
		ImageFormat::WebP => {
			let data = if img.color().has_alpha() {
				let px = img.to_rgba8();
				webp::Encoder::from_rgba(&px, out_w, out_h)
					.encode(WEBP_QUALITY)
					.to_vec()
			} else {
				let px = img.to_rgb8();
				webp::Encoder::from_rgb(&px, out_w, out_h)
					.encode(WEBP_QUALITY)
					.to_vec()
			};
			fs::write(&tmp, data).map_err(|e| internal(&e))?;
		}
		_ => img
			.save_with_format(&tmp, format)
			.map_err(|e| internal(&e))?,
	}
	fs::rename(&tmp, out).map_err(|e| internal(&e))?;
	Ok(())
}

/// About as sharp as the jpegs at 82.
const WEBP_QUALITY: f32 = 80.0;

/// A jpeg has no transparency: what is transparent turns white, not
/// black.
fn on_white(img: DynamicImage) -> RgbImage {
	if !img.color().has_alpha() {
		return img.to_rgb8();
	}
	let (w, h) = (img.width(), img.height());
	let mut white = RgbaImage::from_pixel(w, h, Rgba([255, 255, 255, 255]));
	image::imageops::overlay(&mut white, &img.to_rgba8(), 0, 0);
	DynamicImage::from(white).to_rgb8()
}
