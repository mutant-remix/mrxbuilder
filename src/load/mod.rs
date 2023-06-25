use std::{collections::HashMap, path::PathBuf};

mod colormap;
mod variable;

mod manifest;
use manifest::{Colormap, Emoji, Target};

use crate::Logger;

pub mod svg;

#[derive(Debug)]
pub struct Pack {
    pub colormaps: HashMap<String, Colormap>,
    pub emojis: Vec<Emoji>,
    pub targets: Vec<Target>,
    pub definitions: HashMap<String, String>,
    pub logger: Logger,
}

impl Pack {
    pub fn new(logger: Logger) -> Self {
        Self {
            colormaps: HashMap::new(),
            emojis: Vec::new(),
            targets: Vec::new(),
            definitions: HashMap::new(),
            logger,
        }
    }

    pub fn load_all(&mut self, index_path: &PathBuf) {
        self.logger.info(&format!("Loading index manifest: {:?}", index_path));
        self.load_manifests(index_path);

        self.logger.info(&format!("Loading SVG files"));
        self.load_svgs(); // TODO

        self.logger.info(&format!("Resolving variables"));
        self.resolve_variables(); // TODO

        self.logger.info(&format!("Resolving colormaps"));
        self.resolve_colormaps(); // TODO

        // Clean up
        self.definitions.clear();
        self.colormaps.clear();

        self.logger.info(&format!("Successfully loaded {} emojis", self.emojis.len()));
    }

    pub fn build_all(&mut self) {
        self.logger.info(&format!("Selected {} targets", self.targets.len()));
    }
}
