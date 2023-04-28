#[macro_use]
extern crate log;

use std::{
    fs,
    path::PathBuf,
    collections::HashMap
};
use resvg::{
    render,
    FitTo,
    tiny_skia::{Pixmap, Transform},
    usvg::{Options, Tree, TreeParsing}
};
use image::{RgbaImage};

fn walk_dir(path: &PathBuf, file_paths: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir(&path, file_paths)?;
        } else {
            file_paths.push(path);
        }
    }
    Ok(())
}

fn main() {
    env_logger::init();

    info!("Loading svgs");

    let mut svg_paths: Vec<PathBuf> = Vec::new();

    let root_path = std::env::current_dir().unwrap().join("../assets/svg");
    walk_dir(&root_path, &mut svg_paths).unwrap();

    let mut svg_files: HashMap<PathBuf, String> = HashMap::new();
    for path in svg_paths {
        let svg_file = fs::read_to_string(path.clone()).unwrap();
        svg_files.insert(path, svg_file);
    }

    info!("Rendering svgs");

    let mut rendered_svgs: HashMap<PathBuf, RgbaImage> = HashMap::new();
    for (path, svg_file) in svg_files {
        let tree = Tree::from_str(&svg_file, &Options::default()).unwrap();

        let mut pixmap = Pixmap::new(1024, 1024).unwrap();
        render(
            &tree,
            FitTo::Original,
            Transform::default().pre_scale(32.0, 32.0),
            pixmap.as_mut()
        ).unwrap();

        let data = pixmap.data();
        let rgba_image = RgbaImage::from_raw(
            1024,
            1024,
            data.to_vec()
        ).unwrap();

        debug!("Rendered {:?}", path.file_name());

        rendered_svgs.insert(path, rgba_image);
    }

    let out_path = std::env::current_dir().unwrap().join("./out");
    fs::create_dir_all(&out_path).unwrap();

    for (path, image) in rendered_svgs {
        let png_path = out_path.join(path.file_name().unwrap()).with_extension("png");
        image.save(png_path).unwrap();

        debug!("Saved {:?}", path.file_name());
    }
}
