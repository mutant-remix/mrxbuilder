use image::{RgbaImage};

use rayon::prelude::*;

mod png;
mod avif;
mod webp;

pub enum EncodeTarget {
    Png,
    Avif, // slow and useless and lossy
    Webp,
}

pub fn encode_rasters(rgba: &Vec<RgbaImage>, target: EncodeTarget) -> Vec<Vec<u8>> {
    match target {
        EncodeTarget::Png => {
            // The encoder is parallelised so we don't need to do it here
            rgba.iter().map(|image| {
                png::encode(&image)
            }).collect()
        },
        EncodeTarget::Avif => {
            rgba.par_iter().map(|image| {
                avif::encode(&image)
            }).collect()
        },
        EncodeTarget::Webp => {
            rgba.par_iter().map(|image| {
                webp::encode(&image)
            }).collect()
        }
    }
}
