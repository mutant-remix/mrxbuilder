use image::{codecs::webp::WebPEncoder, RgbaImage};

pub fn encode(rgba: &RgbaImage) -> Vec<u8> {
    let mut buffer = Vec::new();
    let encoder = WebPEncoder::new(&mut buffer);

    encoder
        .encode(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            image::ColorType::Rgba8,
        )
        .unwrap();

    buffer
}
