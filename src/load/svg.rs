use std::{fs, fmt, path::PathBuf};
use resvg::usvg::{Tree, Options, TreeParsing};

use crate::load::Pack;

#[derive(Clone)]
pub struct SvgTree(pub Tree);

impl fmt::Debug for SvgTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &format!("<SvgTree ({}x{})>", self.0.size.width(), self.0.size.height())
        )
    }
}

impl SvgTree {
    fn from_path(path: &PathBuf) -> Self {
        match fs::read_to_string(path) {
            Ok(svg) => match Tree::from_str(&svg, &Options::default()) {
                Ok(tree) => Self(tree),
                Err(err) => panic!("Error parsing SVG file at '{:?}' with error '{}'", path, err),
            },
            Err(err) => panic!("Error reading SVG file at '{:?}' with error '{}'", path, err),
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
