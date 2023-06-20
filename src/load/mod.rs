use std::{collections::HashMap, path::PathBuf};

mod colormap;
mod variable;

mod manifest;
use manifest::{Colormap, Emoji, Target};

pub mod svg;

#[derive(Debug)]
pub struct Pack {
    pub colormaps: HashMap<String, Colormap>,
    pub emojis: Vec<Emoji>,
    pub targets: Vec<Target>,
    pub definitions: HashMap<String, String>,
}

impl Pack {
    pub fn new() -> Self {
        Self {
            colormaps: HashMap::new(),
            emojis: Vec::new(),
            targets: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn load_all(&mut self, index_path: &PathBuf) {
        println!("Loading index manifest: {:?}", index_path);
        self.load_manifests(index_path);

        println!("Loading SVG files");
        self.load_svgs(); // TODO

        println!("Resolving variables");
        self.resolve_variables(); // TODO

        println!("Resolving colormaps");
        self.resolve_colormaps(); // TODO

        // Clean up
        self.definitions.clear();
        self.colormaps.clear();

        println!("Successfully loaded {} emojis", self.emojis.len());
    }

    pub fn build_all(&mut self) {
        println!("Building targets (TODO)");
    }
}
