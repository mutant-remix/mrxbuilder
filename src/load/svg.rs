use resvg::usvg::{Options, Tree, TreeParsing};
use std::{fmt, fs, path::PathBuf};
use svgcleaner::{
    cleaner::{clean_doc, parse_data},
    CleaningOptions, ParseOptions, WriteOptions,
};

use crate::load::Pack;

#[derive(Clone)]
pub struct SvgTree {
    pub tree: Tree,
    pub string: String,
}

impl fmt::Debug for SvgTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!(
            "<SvgTree ({}x{})>",
            self.tree.size.width(),
            self.tree.size.height()
        ))
    }
}

impl SvgTree {
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

                match Tree::from_str(&cleaned_string, &Options::default()) {
                    Ok(tree) => Self {
                        tree,
                        string: cleaned_string,
                    },
                    Err(err) => panic!(
                        "Error parsing (2 stage) SVG file at '{:?}' with error '{}'",
                        path, err
                    ),
                }
            }
            Err(err) => panic!(
                "Error reading SVG file at '{:?}' with error '{}'",
                path, err
            ),
        }
    }

    // pub fn replace_colors(&mut self, map: ) {
    //     let mut new_tree = self.0.clone();
    //     self.0 = new_tree;
    // }
}

impl Pack {
    pub fn load_svgs(&mut self) {
        for emoji in &mut self.emojis {
            let svg_tree = SvgTree::from_path(&emoji.src);
            emoji.svg = Some(svg_tree);
        }
    }
}
