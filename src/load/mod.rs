use std::{
    fs,
    path::PathBuf,
    collections::HashMap
};

mod svgtree;
use svgtree::SvgTree;

mod manifest;
use manifest::{Target, Colormap, Emoji, OutputFormat, OutputStructure, Container, FilenameFormat};

use crate::encode::{ EncodeTarget, OxiPngMode };


#[derive(Debug)]
pub struct Data {
    pub svg: HashMap<PathBuf, SvgTree>,
    pub colormaps: HashMap<String, Colormap>,
    pub emojis: Vec<Emoji>,
    pub targets: Vec<Target>,
    pub definitions: HashMap<String, String>,
}

impl Data {
    pub fn new() -> Self {
        Self {
            svg: HashMap::new(),
            colormaps: HashMap::new(),
            emojis: Vec::new(),
            targets: Vec::new(),
            definitions: HashMap::new(),
        }
    }

    fn load_manifest(path: &PathBuf) -> toml::Value {
        let toml = match fs::read_to_string(path) {
            Ok(manifest) => manifest,
            Err(e) => panic!("Error reading manifest file: {}", e),
        };

        match toml::from_str(&toml) {
            Ok(manifest) => manifest,
            Err(e) => panic!("Error parsing manifest file: {}", e),
        }
    }

    fn load_svg(path: &PathBuf) -> SvgTree {
        match fs::read_to_string(path) {
            Ok(svg) => SvgTree::from_str(&svg),
            Err(e) => panic!("Error reading SVG file: {}", e),
        }
    }

    pub fn load(&mut self, index_path: &PathBuf) {
        // Starting with the index
        let mut queue = vec![index_path.clone()];
        // Load manifest files recursively
        while let Some(manifest_path) = queue.pop() {
            let manifest = Self::load_manifest(&manifest_path);

            // Include
            // If this manifest contains include entries
            if let Some(inclusions) = manifest.get("include") {
                // Turn them into an array
                let includes = inclusions.as_array().unwrap();

                // Flatten and collect the paths
                let mut relative_paths: Vec<&str> = Vec::new();

                for include in includes.iter() {
                    if let Some(paths) = include.get("paths") {
                        let paths = paths.as_array().unwrap();

                        for path in paths.iter() {
                            relative_paths.push(path.as_str().unwrap());
                        }
                    }
                };

                // Add them to the queue
                for relative_path in relative_paths.iter() {
                    let mut new_path = manifest_path.clone();

                    new_path.pop();
                    new_path.push(relative_path);

                    queue.push(new_path);
                }
            }

            // Define
            if let Some(definitions) = manifest.get("define") {
                let definitions = definitions.as_array().unwrap();

                for definition in definitions.iter() {
                    let definition = definition.as_table().unwrap();

                    for (key, value) in definition.iter() {
                        self.definitions.insert(key.to_string(), value.to_string());
                    }
                }
            }

            // Colormap
            if let Some(colormaps) = manifest.get("colormap") {
                let colormaps = colormaps.as_array().unwrap();

                for colormap in colormaps.iter() {
                    let colormap = colormap.as_table().unwrap();

                    let name = match colormap.get("name") {
                        Some(name) => name.to_string(),
                        None => panic!("Colormap is missing a name in {:?}", manifest_path),
                    };

                    let mut label: Option<String> = None;
                    let mut shortcode: Option<String> = None;
                    let mut codepoint: Option<String> = None;

                    let mut entries = HashMap::new();

                    for (key, value) in colormap.iter() {
                        match key.as_str() {
                            "name" => (),
                            "label" => label = Some(value.to_string()),
                            "shortcode" => shortcode = Some(value.to_string()),
                            "codepoint" => codepoint = Some(value.to_string()),
                            _ => {
                                entries.insert(key.to_string(), value.to_string());
                            },
                        }
                    }

                    self.colormaps.insert(name, Colormap {
                        label,
                        shortcode,
                        codepoint,
                        entries,
                    });
                }
            }

            // Emoji
            if let Some(emojis) = manifest.get("emoji") {
                let emojis = emojis.as_array().unwrap();

                for emoji in emojis.iter() {
                    let emoji = emoji.as_table().unwrap();

                    let src = match emoji.get("src") {
                        Some(src) => {
                            let mut full_path = manifest_path.clone();
                            full_path.pop();
                            full_path.push(src.as_str().unwrap());
                            full_path
                        },
                        None => panic!("Emoji is missing a src in {:?}", manifest_path),
                    };

                    let name = match emoji.get("name") {
                        Some(name) => name.to_string(),
                        None => panic!("Emoji is missing a name in {:?}", manifest_path),
                    };

                    let description = match emoji.get("description") {
                        Some(description) => description.to_string(),
                        None => panic!("Emoji is missing a description in {:?}", manifest_path),
                    };

                    let category: Vec<String> = match emoji.get("category") {
                        Some(category) => {
                            category.as_array().unwrap().iter().map(|c| c.to_string()).collect()
                        },
                        None => panic!("Emoji is missing a category in {:?}", manifest_path),
                    };

                    let tags: Vec<String> = match emoji.get("tags") {
                        Some(tags) => {
                            tags.as_array().unwrap().iter().map(|t| t.to_string()).collect()
                        },
                        None => panic!("Emoji is missing tags in {:?}", manifest_path),
                    };

                    let codepoint: Option<Vec<String>> = match emoji.get("codepoint") {
                        Some(codepoint) => {
                            Some(codepoint.as_array().unwrap().iter().map(|c| c.to_string()).collect())
                        },
                        None => None,
                    };

                    let shortcodes: Vec<String> = match emoji.get("shortcode") {
                        Some(shortcode) => {
                            shortcode.as_array().unwrap().iter().map(|s| s.to_string()).collect()
                        },
                        None => vec![],
                    };

                    let colormaps: Vec<String> = match emoji.get("colormaps") {
                        Some(colormaps) => {
                            colormaps.as_array().unwrap().iter().map(|c| c.to_string()).collect()
                        },
                        None => vec![],
                    };

                    self.emojis.push(Emoji {
                        src,
                        name,
                        description,
                        category,
                        tags,
                        codepoint,
                        shortcodes,
                        colormaps,
                    });
                }
            }

            // Target
            if let Some(targets) = manifest.get("target") {
                let targets = targets.as_array().unwrap();

                for target in targets.iter() {
                    let target = target.as_table().unwrap();

                    let name = match target.get("name") {
                        Some(name) => name.to_string(),
                        None => panic!("Target is missing 'name' in {:?}", manifest_path),
                    };

                    let tags: Vec<String> = match target.get("tags") {
                        Some(tags) => {
                            tags.as_array().unwrap().iter().map(|t| t.to_string()).collect()
                        },
                        None => panic!("Target is missing 'tags' in {:?}", manifest_path),
                    };

                    let include_tags: Vec<String> = match target.get("include_tags") {
                        Some(tags) => {
                            tags.as_array().unwrap().iter().map(|t| t.to_string()).collect()
                        },
                        None => panic!("Target is missing 'include_tags' in {:?}", manifest_path),
                    };

                    let output_structure = match target.get("structure") {
                        Some(structure) => {
                            let structure = structure.as_table().unwrap();

                            let container = match structure.get("container") {
                                Some(container) => match container.as_str().unwrap() {
                                    "tar.gz" => Container::TarGz,
                                    "zip" => Container::Zip,
                                    "directory" => Container::Directory,
                                    _ => panic!("Target contains unknown 'structure.container' in {:?}", manifest_path),
                                }
                                _ => panic!("Target is missing 'structure.container' in {:?}", manifest_path),
                            };

                            let filenames = match structure.get("filenames") {
                                Some(filenames) => match filenames.as_str().unwrap() {
                                    "shortcode" => FilenameFormat::Shortcode,
                                    "codepoint" => FilenameFormat::Codepoint,
                                    _ => panic!("Target contains unknown 'structure.filenames' in {:?}", manifest_path),
                                }
                                _ => panic!("Target is missing 'structure.filenames' in {:?}", manifest_path),
                            };

                            let subdirectories = match structure.get("subdirectories") {
                                Some(subdirectories) =>
                                    subdirectories
                                    .as_bool()
                                    .expect(format!("Target contains invalid 'structure.subdirectories' in {:?}", manifest_path).as_str()),
                                None => panic!("Target is missing 'structure.subdirectories' in {:?}", manifest_path),
                            };

                            OutputStructure {
                                container,
                                filenames,
                                subdirectories,
                            }
                        },
                        None => panic!("Target is missing 'structure' in {:?}", manifest_path),
                    };

                    let output_format = match target.get("output") {
                        Some(output) => {
                            let output = output.as_table().unwrap();

                            let size = match output.get("size") {
                                Some(size) => match size.as_integer() {
                                    Some(size) => {
                                        if size < 0 {
                                            panic!("Target contains negative 'output.size' in {:?}", manifest_path);
                                        }

                                        if size > 65536 {
                                            panic!("Target contains 'output.size' over 65536 in {:?}", manifest_path);
                                        }

                                        Some(size as u32)
                                    },
                                    None => panic!("Target contains invalid 'output.size' in {:?}", manifest_path),
                                },
                                None => None,
                            };

                            let compression = match output.get("compression") {
                                Some(compression) => match compression.as_float() {
                                    Some(compression) => {
                                        if compression < 0.0 {
                                            panic!("Target contains negative 'output.compression' in {:?}", manifest_path);
                                        }

                                        Some(compression)
                                    },
                                    None => panic!("Target contains invalid 'output.compression' (must contain a decimal point) in {:?}", manifest_path),
                                },
                                None => None,
                            };

                            match output.get("format") {
                                Some(format) => match format.as_str().unwrap() {
                                    "svg" => OutputFormat::Svg,
                                    "png" => OutputFormat::Raster {
                                        format: EncodeTarget::PngImage,
                                        size: size.unwrap() as u32,
                                    },
                                    "png-mtpng" => {
                                        match compression {
                                            Some(compression) => {
                                                if compression > 3.0 {
                                                    panic!("Target uses 'png-mtpng', but contains 'output.compression' over 3 in {:?}", manifest_path);
                                                }

                                                OutputFormat::Raster {
                                                    format: EncodeTarget::PngMtpng(compression as u8),
                                                    size: size.unwrap() as u32,
                                                }
                                            },
                                            None => panic!("Target uses 'png-mtpng', but doesn't specify 'output.compression' in {:?}", manifest_path),
                                        }
                                    },
                                    "png-oxipng-zopfli" => {
                                        match compression {
                                            Some(compression) => {
                                                if compression > 15.0 {
                                                    panic!("Target uses 'png-oxipng-zopfli', but contains 'output.compression' over 15 in {:?}", manifest_path);
                                                }

                                                OutputFormat::Raster {
                                                    format: EncodeTarget::PngOxipng(OxiPngMode::Zopfli(compression as u8)),
                                                    size: size.unwrap() as u32,
                                                }
                                            },
                                            None => panic!("Target uses 'png-oxipng-zopfli', but doesn't specify 'output.compression' in {:?}", manifest_path),
                                        }
                                    }
                                    "png-oxipng-libdeflater" => {
                                        match compression {
                                            Some(compression) => {
                                                if compression > 15.0 {
                                                    panic!("Target uses 'png-oxipng-libdeflater', but contains 'output.compression' over 12 in {:?}", manifest_path);
                                                }

                                                OutputFormat::Raster {
                                                    format: EncodeTarget::PngOxipng(OxiPngMode::Libdeflater(compression as u8)),
                                                    size: size.unwrap() as u32,
                                                }
                                            },
                                            None => panic!("Target uses 'png-oxipng-libdeflater', but doesn't specify 'output.compression' in {:?}", manifest_path),
                                        }
                                    }
                                    "webp" => OutputFormat::Raster {
                                        format: EncodeTarget::Webp,
                                        size: size.unwrap() as u32,
                                    },
                                    "avif-lossy" => {
                                        match compression {
                                            Some(compression) => {
                                                if compression > 100.0 {
                                                    panic!("Target uses 'avif-lossy', but contains 'output.compression' over 100.0 in {:?}", manifest_path);
                                                }

                                                OutputFormat::Raster {
                                                    format: EncodeTarget::Avif { quality: compression as f32, speed: 1 },
                                                    size: size.unwrap() as u32,
                                                }
                                            },
                                            None => panic!("Target uses 'avif-lossy', but doesn't specify 'output.compression' in {:?}", manifest_path),
                                        }
                                    }
                                    _ => panic!("Target contains unknown output.format in {:?}", manifest_path),
                                },
                                None => panic!("Target is missing format.format in {:?}", manifest_path),
                            }
                        },
                        None => panic!("Target is missing format in {:?}", manifest_path),
                    };

                    self.targets.push(Target {
                        name,
                        tags,
                        include_tags,
                        output_structure,
                        output_format,
                    });
                }
            }
        }

        // resolve variables

        println!("{:#?}", self);
    }
}
