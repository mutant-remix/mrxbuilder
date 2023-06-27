use rayon::prelude::*;
use resvg::usvg::{Options, Tree, TreeParsing, TreeWriting, XmlOptions, Paint, Color};
use std::{fmt, fs, path::PathBuf};
use svgcleaner::{
    cleaner::{clean_doc, parse_data},
    CleaningOptions, ParseOptions, WriteOptions,
};

use crate::load::Pack;

#[derive(Clone)]
pub struct Svg(pub String);

impl fmt::Debug for Svg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<SvgTree>")
    }
}

impl Svg {
    fn from_path(path: &PathBuf) -> Self {
        match fs::read_to_string(path) {
            Ok(svg) => {
                let mut svgcleaner_doc = match parse_data(&svg, &ParseOptions::default()) {
                    Ok(doc) => doc,
                    Err(err) => panic!(
                        "Error parsing (1 stage) SVG file at '{:?}' with error '{}'",
                        path, err
                    ),
                };

                let cleaned_string = match clean_doc(
                    &mut svgcleaner_doc,
                    &CleaningOptions::default(),
                    &WriteOptions::default(),
                ) {
                    Ok(_) => svgcleaner_doc.to_string(),
                    Err(err) => panic!(
                        "Error cleaning SVG file at '{:?}' with error '{}'",
                        path, err
                    ),
                };

                Self(cleaned_string)
            }
            Err(err) => panic!(
                "Error reading SVG file at '{:?}' with error '{}'",
                path, err
            ),
        }
    }

    pub fn replace_colors(&mut self, map: Vec<(Color, Color)>) {
        let tree = Tree::from_str(&self.0, &Options::default()).unwrap();

        tree.paint_servers(|paint| {
            match paint {
                Paint::Color(color) => {
                    let mut color = *color;

                    for (from, to) in map.iter() {
                        if color == *from {
                            color = *to;
                        }
                    }
                },
                _ => {},
            };
        });

        self.0 = tree.to_string(&XmlOptions::default());
    }
}

impl Pack {
    pub fn load_svgs(&mut self) {
        self.emojis.par_iter_mut().for_each(|emoji| {
            let svg_tree = Svg::from_path(&emoji.src);
            emoji.svg = Some(svg_tree);
        });
    }
}
