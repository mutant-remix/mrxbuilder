use std::path::PathBuf;

mod colormap;
mod variable;

pub mod manifest;
use manifest::Emoji;

use crate::Pack;

pub mod svg;

impl Pack {
    pub fn load_all(&mut self, index_path: &PathBuf) {
        self.logger.info("Loading build files");
        let mut stage = self.logger.new_stage("Loading", 5);

        self.logger
            .load(&format!("Loading index manifest: {:?}", index_path));
        stage.inc();
        self.load_manifests(index_path);

        self.logger.load(&format!("Loading and cleaning SVG files"));
        stage.inc();
        self.load_svgs();

        self.logger.load(&format!("Resolving variables"));
        stage.inc();
        self.resolve_variables();

        self.logger.load(&format!("Resolving colormaps"));
        stage.inc();
        self.resolve_colormaps();

        // Clean up
        self.definitions.clear();
        self.colormaps.clear();

        self.logger
            .load(&format!("Successfully loaded {} emojis", self.emojis.len()));
    }
}
