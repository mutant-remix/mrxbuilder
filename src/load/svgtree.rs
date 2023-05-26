use std::fmt;
use resvg::usvg::{Tree, Options, TreeParsing};

pub struct SvgTree(pub Tree);
impl fmt::Debug for SvgTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &format!("<SvgTree ({})>",self.0.size.to_string())
        )
    }
}

impl SvgTree {
    pub fn from_str(svg: &str) -> Self {
        let tree = Tree::from_str(&svg, &Options::default()).unwrap();
        Self(tree)
    }

    pub fn replace_colors() {
        unimplemented!()
    }
}
