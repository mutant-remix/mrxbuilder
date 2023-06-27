mod clean;
use crate::load::svg::Svg;
use clean::clean_svg;

mod rasterize;
use rasterize::rasterise_svg;

mod encode;
use core::num::NonZeroU8;
use encode::{avif, png_image, png_oxipng, webp};
use oxipng::Deflaters;

#[derive(Debug)]
pub enum OxiPngMode {
    Libdeflater(u8), // 0-12
    Zopfli(u8),      // 0-15
}

#[derive(Debug)]
pub enum EncodeTarget {
    PngImage,
    PngOxipng(OxiPngMode),
    Avif {
        quality: f32, // 100.0-0.0
        speed: u8,    // 1-10
    },
    Webp,
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

type Raster = Vec<u8>;
pub fn encode_svg(svg: &Svg, size: u32, target: EncodeTarget) -> Raster {
    let svg = clean_svg(svg);
    let raster = rasterise_svg(&svg.0, size);

    match target {
        EncodeTarget::PngOxipng(oxipng_mode) => match oxipng_mode {
            OxiPngMode::Libdeflater(compression) => {
                png_oxipng::encode(&raster, Deflaters::Libdeflater { compression })
            }
            OxiPngMode::Zopfli(iterations) => png_oxipng::encode(
                &raster,
                Deflaters::Zopfli {
                    iterations: NonZeroU8::new(iterations).unwrap(),
                },
            ),
        },
        EncodeTarget::PngImage => png_image::encode(&raster),
        EncodeTarget::Avif { quality, speed } => avif::encode(&raster, quality, speed),
        EncodeTarget::Webp => webp::encode(&raster),
    }
}
