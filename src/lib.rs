use std::path::{Path,PathBuf};
use std::ffi::OsStr;
use image::{GenericImageView, ImageReader, imageops};
use anyhow::Result;

pub mod args;
pub mod common;
pub mod config;
pub mod constants;
pub mod params;
pub mod types;

pub fn run(param: &params::Params) -> Result<()> {
    let mut img = ImageReader::open(&param.imagepath)?
        .with_guessed_format()?
        .decode()?;
    if !param.expand && img.width().min(img.height()) <= param.size {
        img = crop_image(img, param.position);
    } else {
        img = crop_scale_image(img, param.position, param.size);
    }
    let outfile = make_outfile_path(param);
    img.save(outfile)?;
    Ok(())
}

fn crop_scale_image(image: image::DynamicImage, position: types::Position, size: u32) -> image::DynamicImage {
    match position {
        types::Position::Start => {
            let (width, height) = image.dimensions();
            let rimg = if width < height {
                image.resize(size, u32::MAX, imageops::FilterType::Lanczos3)
            } else {
                image.resize(u32::MAX, size, imageops::FilterType::Lanczos3)
            };
            rimg.crop_imm(0, 0, size, size)
        },
        types::Position::Center => {
            image.resize_to_fill(size, size, imageops::FilterType::Lanczos3)

        },
        types::Position::End => {
            let (width, height) = image.dimensions();
            if width < height {
                let rimg = image.resize(size, u32::MAX, imageops::FilterType::Lanczos3);
                rimg.crop_imm(0, rimg.height() - size, size, size)
            } else {
                let rimg = image.resize(u32::MAX, size, imageops::FilterType::Lanczos3);
                rimg.crop_imm(rimg.width() - size, 0, size, size)
            }
        },
    }
}

fn crop_image(image: image::DynamicImage, position: types::Position) -> image::DynamicImage {
    let (width, height) = image.dimensions();
    let size = width.min(height);
    match position {
        types::Position::Start => {
            image.crop_imm(0, 0, size, size)
        },
        types::Position::Center => {
            image.crop_imm((width - size) / 2, (height - size) / 2, size, size)
        },
        types::Position::End => {
            image.crop_imm(width - size, height - size, size, size)
        },
    }
}

fn make_outfile_path(param: &params::Params) -> PathBuf {
    let img_path = Path::new(&param.imagepath);
    let img_file = img_path.file_stem().unwrap_or(OsStr::new("")).to_string_lossy();
    let img_ext = img_path.extension().unwrap_or(OsStr::new("")).to_string_lossy();
    let out_path = Path::new(&param.outdir);
    if param.outimage.is_empty() {
        if img_ext.is_empty() {
            common::get_unique_path(&out_path.join(format!("{}_{}", &img_file, param.size)))
        }
        else {
            common::get_unique_path(&out_path.join(format!("{}_{}.{}", &img_file, param.size, img_ext)))
        }
    } else {
        common::get_unique_path(&out_path.join(&param.outimage))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, Rgba, RgbaImage};

    fn sample_image() -> DynamicImage {
        let mut img = RgbaImage::new(4, 6);

        for y in 0..6 {
            for x in 0..4 {
                let c = if y < 2 {
                    Rgba([255, 0, 0, 255])
                } else if y < 4 {
                    Rgba([0, 255, 0, 255])
                } else {
                    Rgba([0, 0, 255, 255])
                };
                img.put_pixel(x, y, c);
            }
        }

        DynamicImage::ImageRgba8(img)
    }

    #[test]
    fn crop_image_center_makes_square() {
        let img = sample_image();
        let out = crop_image(img, types::Position::Center);
        assert_eq!(out.dimensions(), (4, 4));
    }

    #[test]
    fn crop_scale_image_returns_requested_size() {
        let img = sample_image();
        let out = crop_scale_image(img, types::Position::Start, 8);
        assert_eq!(out.dimensions(), (8, 8));
    }
}
