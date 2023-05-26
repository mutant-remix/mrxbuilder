use image::{RgbaImage};
use oxipng::Deflaters;
use core::num::NonZeroU8;
use rayon::prelude::*;
use ml_progress::progress;

mod png_mtpng;
mod png_oxipng;
mod png_image;
mod avif;
mod webp;

#[derive(Debug)]
pub enum OxiPngMode {
    Libdeflater(u8), // 0-12
    Zopfli(u8), // 0-15
}

#[derive(Debug)]
pub enum EncodeTarget {
    PngImage,
    PngMtpng(u8), // 0-3
    PngOxipng(OxiPngMode),
    Avif {
        quality: f32, // 100.0-0.0
        speed: u8, // 1-10
    },
    Webp,
}

pub fn encode_rasters(rgba: &Vec<RgbaImage>, target: EncodeTarget) -> Vec<Vec<u8>> {
    println!("Encoding {} images ({}x{}) to {:?}", rgba.len(), rgba[0].width(), rgba[0].height(), target);

    let progress = progress!(
        rgba.len();
        "[" percent "] " pos "/" total " " bar_fill " (ETA " eta_hms ")"
    ).unwrap();

    let output = match target {
        // oxipng and mtpng hang with rayon
        EncodeTarget::PngOxipng(oxipng_mode) => match oxipng_mode {
            OxiPngMode::Libdeflater(compression) => {
                rgba.iter().map(|image| {
                    progress.inc(1);
                    png_oxipng::encode(&image, Deflaters::Libdeflater { compression })
                }).collect()
            },
            OxiPngMode::Zopfli(iterations) => {
                rgba.par_iter().map(|image| {
                    progress.inc(1);
                    png_oxipng::encode(&image, Deflaters::Zopfli { iterations: NonZeroU8::new(iterations).unwrap() })
                }).collect()
            },
        },
        EncodeTarget::PngMtpng(compression) => {
            rgba.iter().map(|image| {
                progress.inc(1);
                png_mtpng::encode(&image, compression)
            }).collect()
        },
        EncodeTarget::PngImage => {
            rgba.par_iter().map(|image| {
                progress.inc(1);
                png_image::encode(&image)
            }).collect()
        },
        EncodeTarget::Avif { quality, speed } => {
            rgba.par_iter().map(|image| {
                progress.inc(1);
                avif::encode(&image, quality, speed)
            }).collect()
        },
        EncodeTarget::Webp => {
            rgba.par_iter().map(|image| {
                progress.inc(1);
                webp::encode(&image)
            }).collect()
        }
    };

    progress.finish_and_clear();

    return output;
}
