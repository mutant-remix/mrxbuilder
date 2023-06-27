use image::RgbaImage;
use ravif::{Encoder, Img, RGBA8};
use std::mem::transmute;

pub fn encode(rgba: &RgbaImage, quality: f32, speed: u8) -> Vec<u8> {
    let encoder = Encoder::new().with_quality(quality).with_speed(speed);

    let img = Img::new(
        unsafe { transmute::<&[u8], &[RGBA8]>(rgba.as_raw()) },
        rgba.width() as usize,
        rgba.height() as usize,
    );

    let encoded = encoder.encode_rgba(img).unwrap();

    encoded.avif_file
}
