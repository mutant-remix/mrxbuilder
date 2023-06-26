use std::{collections::HashMap, path::PathBuf};

mod colormap;
mod variable;

mod manifest;
use kdam::BarExt;
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
        self.logger.load(&format!("Loading index manifest: {:?}", index_path));
        self.load_manifests(index_path);

        self.logger.load(&format!("Loading and cleaning SVG files"));
        self.load_svgs();

        self.logger.load(&format!("Resolving variables"));
        self.resolve_variables();

        self.logger.load(&format!("Resolving colormaps"));
        self.resolve_colormaps();

        // Clean up
        self.definitions.clear();
        self.colormaps.clear();

        self.logger.load(&format!("Successfully loaded {} emojis", self.emojis.len()));
    }

    pub fn build_all(&mut self) {
        self.logger.info(&format!("Selected {} targets", self.targets.len()));
        self.logger.set_stage_count(self.targets.len() * 3);
        self.logger.total_bar.as_mut().unwrap().update(1);
    }
}
