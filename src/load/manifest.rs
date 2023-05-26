use std::{
    path::PathBuf,
    collections::HashMap
};

use crate::encode::EncodeTarget;

#[derive(Debug)]
pub enum Container {
    Directory,
    TarGz,
    Zip,
}

#[derive(Debug)]
pub enum FilenameFormat {
    Shortcode,
    Codepoint
}

#[derive(Debug)]
pub struct OutputStructure {
    pub container: Container,
    pub filenames: FilenameFormat,
    pub subdirectories: bool,
}

#[derive(Debug)]
pub enum OutputFormat {
    Svg,
    Raster { format: EncodeTarget, size: u32 },
}

#[derive(Debug)]
pub struct Target {
    pub name: String,
    pub tags: Vec<String>,
    pub include_tags: Vec<String>,
    pub output_structure: OutputStructure,
    pub output_format: OutputFormat,
}

#[derive(Debug)]
pub struct Colormap {
    pub label: Option<String>,
    pub shortcode: Option<String>,
    pub codepoint: Option<String>,
    pub entries: HashMap<String, String>,
}

#[derive(Debug)]
pub struct Emoji {
    pub src: PathBuf,
    pub name: String,
    pub category: Vec<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub codepoint: Option<Vec<String>>,
    pub shortcodes: Vec<String>,
    pub colormaps: Vec<String>,
}
