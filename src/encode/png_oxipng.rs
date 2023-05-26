use image::{codecs::png::PngEncoder, ImageEncoder, RgbaImage};
use oxipng::{optimize_from_memory, Options, Deflaters};


pub fn encode(rgba: &RgbaImage, deflater: Deflaters) -> Vec<u8> {
    let mut buffer = Vec::new();

    let encoder = PngEncoder::new(&mut buffer);
    encoder
        .write_image(&rgba, rgba.width(), rgba.height(), image::ColorType::Rgba8)
        .unwrap();

    let oxipng_options = Options {
        deflate: deflater,
        ..Default::default()
    };

    optimize_from_memory(&buffer, &oxipng_options).unwrap()
}
