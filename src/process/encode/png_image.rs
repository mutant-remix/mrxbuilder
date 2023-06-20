use image::{codecs::png::PngEncoder, ImageEncoder, RgbaImage};

pub fn encode(rgba: &RgbaImage) -> Vec<u8> {
    let mut buffer = Vec::new();

    let encoder = PngEncoder::new(&mut buffer);
    encoder
        .write_image(&rgba, rgba.width(), rgba.height(), image::ColorType::Rgba8)
        .unwrap();

    buffer
}
