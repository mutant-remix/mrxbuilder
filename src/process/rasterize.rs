use resvg::{
    render,
    tiny_skia::{Pixmap, Transform},
    usvg::{Tree, TreeParsing},
    FitTo,
};

use image::RgbaImage;

pub fn rasterise_svg(svg: &str, size: u32) -> RgbaImage {
    let tree = Tree::from_str(svg, &Default::default()).unwrap();

    let mut pixmap = Pixmap::new(size, size).unwrap();
    render(
        &tree,
        FitTo::Original,
        Transform::default().pre_scale(
            size as f32 / tree.size.width() as f32,
            size as f32 / tree.size.height() as f32,
        ),
        pixmap.as_mut(),
    )
    .unwrap();

    let data = pixmap.data();
    let rgba_image = RgbaImage::from_raw(size, size, data.to_vec()).unwrap();

    rgba_image
}
