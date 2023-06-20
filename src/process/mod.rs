mod clean;
use crate::load::svg::SvgTree;
use clean::clean_svg;

mod rasterize;
use rasterize::rasterise_svg;

mod encode;
use core::num::NonZeroU8;
use oxipng::Deflaters;
use encode::{avif, png_image, png_oxipng, webp};

#[derive(Debug)]
pub enum OxiPngMode {
    Libdeflater(u8), // 0-12
    Zopfli(u8), // 0-15
}

#[derive(Debug)]
pub enum EncodeTarget {
    PngImage,
    PngOxipng(OxiPngMode),
    Avif {
        quality: f32, // 100.0-0.0
        speed: u8, // 1-10
    },
    Webp,
}

pub fn encode_svg(svg: &SvgTree, size: u32, target: EncodeTarget) -> Vec<u8> {
    let svg = clean::clean_svg(svg);
    let raster = rasterise_svg(&svg.0, size);

    match target {
        EncodeTarget::PngOxipng(oxipng_mode) => match oxipng_mode {
            OxiPngMode::Libdeflater(compression) => {
                png_oxipng::encode(&raster, Deflaters::Libdeflater { compression })
            },
            OxiPngMode::Zopfli(iterations) => {
                png_oxipng::encode(&raster, Deflaters::Zopfli { iterations: NonZeroU8::new(iterations).unwrap() })
            },
        },
        EncodeTarget::PngImage => {
            png_image::encode(&raster)
        },
        EncodeTarget::Avif { quality, speed } => {
            avif::encode(&raster, quality, speed)
        },
        EncodeTarget::Webp => {
            webp::encode(&raster)
        }
    }
}
