#![allow(dead_code)]

use std::path::PathBuf;

use stb_image::image;
use stb_image::image::LoadResult;

use super::Texture;

use crate::color::Color;
use crate::geometry::Point3;

pub struct ImageTexture {
    data: Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: usize = 3;

    pub fn new() -> ImageTexture {
        let filename = PathBuf::from("/rust/raytracer/src/texture/earthmap.jpg");

        match image::load(filename) {
            LoadResult::Error(s) => panic!("Error loading image: {}", s),
            LoadResult::ImageF32(_) => panic!("Sorry, can't hangle f32 image format (yet)"),
            LoadResult::ImageU8(img) => ImageTexture {
                data: img.data,
                height: img.height,
                width: img.width,
                bytes_per_scanline: img.width * ImageTexture::BYTES_PER_PIXEL,
            },
        }
    }

    fn clamp(f: f64, min: f64, max: f64) -> f64 {
        if f > max {
            max
        } else if f < min {
            min
        } else {
            f
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Color {
        // use cyan to debug if we have no texture data
        if self.data.is_empty() {
            return Color::new(0., 1., 1.);
        }

        let u = ImageTexture::clamp(u, 0., 1.);
        let i = if u >= 1.0 {
            self.width - 1
        } else {
            (u * (self.width as f64)) as usize
        };

        let v = 1. - ImageTexture::clamp(v, 0., 1.);
        let j = if v >= 1.0 {
            self.height - 1
        } else {
            (v * (self.height as f64)) as usize
        };

        const COLOR_SCALE: f64 = 1. / 255.;
        // let pixel_idx = j * self.bytes_per_scanline + i * ImageTexture::BYTES_PER_PIXEL;
        let pixel_idx = ImageTexture::BYTES_PER_PIXEL * (i + self.width * j);
        Color::new(
            COLOR_SCALE * (self.data[pixel_idx] as f64),
            COLOR_SCALE * (self.data[pixel_idx + 1] as f64),
            COLOR_SCALE * (self.data[pixel_idx + 2] as f64),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let _t = ImageTexture::new();
    }
}
