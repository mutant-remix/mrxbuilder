#[macro_use]
extern crate paris;

use rayon::prelude::*;

mod load;
use load::Data;

mod rasterise;
use rasterise::rasterise_svg;

mod encode;
use encode::{encode_rasters, EncodeTarget};

use std::{
    fs,
    time::Instant,
};

use image::{RgbaImage};

fn main() {
    info!("=> Loading svgs");
    let mut data = Data::new();
    data.load();

    fs::create_dir_all("./out").unwrap();

    // Rasterise
    let start = Instant::now();
    info!("=> Rasterising {} svgs to 256x256", data.svg.len());

    let rasters: Vec<RgbaImage> = data.svg.par_iter().map(|(_, svg)| {
        rasterise_svg(svg, 256)
    }).collect();

    info!("   Took {}s", start.elapsed().as_secs_f32());

    // Encode png
    let start = Instant::now();
    info!("=> Encoding png");

    let _png: Vec<Vec<u8>> = encode_rasters(&rasters, EncodeTarget::Png);

    info!("   Took {}s", start.elapsed().as_secs_f32());
    info!("   Size: {}MiB", _png.iter().map(|image| {
        image.len() as f32 / 1024.0 / 1024.0
    }).sum::<f32>());

    // Encode webp
    let start = Instant::now();
    info!("=> Encoding webp");

    let _webp: Vec<Vec<u8>> = encode_rasters(&rasters, EncodeTarget::Webp);

    info!("   Took {}s", start.elapsed().as_secs_f32());
    info!("   Size: {}MiB", _webp.iter().map(|image| {
        image.len() as f32 / 1024.0 / 1024.0
    }).sum::<f32>());
}
