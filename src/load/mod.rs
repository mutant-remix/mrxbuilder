use std::{collections::HashMap, path::PathBuf};
use rayon::prelude::*;

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
        self.logger.info("Loading build files");
        let mut stage = self.logger.new_stage("Loading", 4);

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

    pub fn build_all(&mut self) {
        self.logger.info(&format!("Building {} targets", self.targets.len()));
        self.logger.set_stage_count(self.targets.len() * 3 + 1);

        for target in &self.targets {
            self.logger.build(&format!("TODO: Building target '{}'", target.name));

            let stage = self.logger.new_stage("Encoding", 500);

            (0..500).collect::<Vec<_>>().par_iter().for_each(|_| {
                stage.clone().inc();
                std::thread::sleep(std::time::Duration::from_millis(100));
            });

            let mut stage = self.logger.new_stage("Packing", 1);
            stage.inc();
            std::thread::sleep(std::time::Duration::from_millis(500));

            stage.inc();
        }
    }
}
