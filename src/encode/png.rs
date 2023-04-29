use image::RgbaImage;
use mtpng::{
    encoder::{Encoder, Options},
    ColorType, Header, CompressionLevel
};

pub fn encode(rgba: &RgbaImage) -> Vec<u8> {
    let mut buffer = Vec::new();

    let mut options = Options::new();
    options.set_compression_level(CompressionLevel::High).unwrap();

    let mut encoder = Encoder::new(&mut buffer, &options);

    let mut header = Header::new();
    header.set_size(rgba.width(), rgba.height()).unwrap();
    header.set_color(ColorType::TruecolorAlpha, 8).unwrap();

    encoder.write_header(&header).unwrap();
    encoder.write_image_rows(&rgba).unwrap();
    encoder.finish().unwrap();

    buffer
}
