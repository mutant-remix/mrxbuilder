use core::num::NonZeroU8;
use oxipng::Deflaters;
use image::RgbaImage;

pub mod avif;
pub mod png_image;
pub mod png_oxipng;
pub mod webp;

#[derive(Clone, Debug)]
pub enum OxiPngMode {
    Libdeflater(u8), // 0-12
    Zopfli(u8),      // 0-15
}

#[derive(Clone, Debug)]
pub enum EncodeTarget {
    PngImage,
    PngOxipng(OxiPngMode),
    Avif {
        quality: f32, // 100.0-0.0
        speed: u8,    // 1-10
    },
    Webp
}

impl EncodeTarget {
    pub fn to_extension(&self) -> &'static str {
        match self {
            EncodeTarget::PngImage => "png",
            EncodeTarget::PngOxipng(_) => "png",
            EncodeTarget::Avif { .. } => "avif",
            EncodeTarget::Webp => "webp",
        }
    }
}

pub fn encode_raster(raster: &RgbaImage, size: u32, target: &EncodeTarget) -> Vec<u8> {
    match target {
        EncodeTarget::PngOxipng(oxipng_mode) => match oxipng_mode {
            OxiPngMode::Libdeflater(compression) => {
                png_oxipng::encode(&raster, Deflaters::Libdeflater { compression: *compression })
            }
            OxiPngMode::Zopfli(iterations) => png_oxipng::encode(
                &raster,
                Deflaters::Zopfli {
                    iterations: NonZeroU8::new(*iterations).unwrap(),
                },
            ),
        },
        EncodeTarget::PngImage => png_image::encode(&raster),
        EncodeTarget::Avif { quality, speed } => avif::encode(&raster, *quality, *speed),
        EncodeTarget::Webp => webp::encode(&raster),
    }
}
