use std::collections::HashMap;
use std::path::PathBuf;
use crate::load::manifest::{Colormap, Emoji, Target};
use crate::process::cache::Cache;
use crate::Logger;

#[derive(Debug)]
pub struct Pack {
    pub cache: Cache,
    pub colormaps: HashMap<String, Colormap>,
    pub emojis: Vec<Emoji>,
    pub targets: Vec<Target>,
    pub definitions: HashMap<String, String>,
    pub output_path: PathBuf,
    pub logger: Logger,
}

impl Pack {
    pub fn new(logger: Logger, output_path: PathBuf) -> Self {
        Self {
            cache: Cache::new(&output_path),
            colormaps: HashMap::new(),
            emojis: Vec::new(),
            targets: Vec::new(),
            definitions: HashMap::new(),
            output_path,
            logger,
        }
    }
}
