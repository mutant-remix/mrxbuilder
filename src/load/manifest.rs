use std::{collections::HashMap, fs, path::PathBuf};
use image::Rgb;

use crate::process::{EncodeTarget, OxiPngMode};
use crate::load::{Pack, svg::SvgTree};

#[derive(Debug)]
pub enum Container {
    Directory,
    TarGz,
    Zip,
}

#[derive(Debug)]
pub enum FilenameFormat {
    Shortcode,
    Codepoint,
}

#[derive(Debug)]
pub struct OutputStructure {
    pub container: Container,
    pub filenames: FilenameFormat,
    pub subdirectories: bool,
}

#[derive(Debug)]
pub enum OutputFormat {
    None,
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
    pub description: Option<String>,
    pub entries: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Emoji {
    pub src: PathBuf,
    pub svg: Option<SvgTree>,
    pub name: String,
    pub category: Vec<String>,
    pub description: String,
    pub tags: Vec<String>,
    pub codepoint: Option<Vec<String>>,
    pub shortcodes: Vec<String>,
    pub colormaps: Vec<String>,
    pub colormap_entries: HashMap<Rgb<u8>, Rgb<u8>>,
}

impl Pack {
    fn load_manifest(path: &PathBuf) -> toml::Value {
        let toml = match fs::read_to_string(path) {
            Ok(manifest) => manifest,
            Err(err) => panic!("Error reading manifest file: {}", err),
        };

        match toml::from_str(&toml) {
            Ok(manifest) => manifest,
            Err(err) => panic!("Error parsing manifest file: {}", err),
        }
    }

    pub fn load_manifests(&mut self, index_path: &PathBuf) {
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
                }

                // Add them to the queue
                for relative_path in relative_paths.iter() {
                    let mut new_path = manifest_path.clone();

                    new_path.pop();
                    new_path.push(relative_path);

                    let new_path = match new_path.canonicalize() {
                        Ok(new_path) => new_path,
                        Err(err) => panic!("Could not find manifest at '{:?}' with error '{}' included in '{:?}'", new_path, err, manifest_path),
                    };

                    queue.push(new_path);
                }
            }

            // Define
            if let Some(definitions) = manifest.get("define") {
                let definitions = definitions.as_array().unwrap();

                for definition in definitions.iter() {
                    let definition = definition.as_table().unwrap();

                    for (key, value) in definition.iter() {
                        let value = match value.as_str() {
                            Some(value) => value,
                            None => panic!("Define value is not a string in {:?}", manifest_path),
                        };

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
                        Some(name) => match name.as_str() {
                            Some(name) => name.to_string(),
                            None => panic!("Colormap name is not a string in {:?}", manifest_path),
                        },
                        None => panic!("Colormap is missing 'name' in {:?}", manifest_path),
                    };

                    let mut label: Option<String> = None;
                    let mut shortcode: Option<String> = None;
                    let mut codepoint: Option<String> = None;
                    let mut description: Option<String> = None;

                    let mut entries = HashMap::new();

                    for (key, value) in colormap.iter() {
                        match key.as_str() {
                            "name" => (),
                            "label" => {
                                label = match value.as_str() {
                                    Some(label) => Some(label.to_string()),
                                    None => panic!(
                                        "Colormap label is not a string in {:?}",
                                        manifest_path
                                    ),
                                }
                            }
                            "shortcode" => {
                                shortcode = match value.as_str() {
                                    Some(shortcode) => Some(shortcode.to_string()),
                                    None => panic!(
                                        "Colormap shortcode is not a string in {:?}",
                                        manifest_path
                                    ),
                                }
                            }
                            "codepoint" => {
                                codepoint = match value.as_str() {
                                    Some(codepoint) => Some(codepoint.to_string()),
                                    None => panic!(
                                        "Colormap codepoint is not a string in {:?}",
                                        manifest_path
                                    ),
                                }
                            }
                            "description" => {
                                description = match value.as_str() {
                                    Some(description) => Some(description.to_string()),
                                    None => panic!(
                                        "Colormap description is not a string in {:?}",
                                        manifest_path
                                    ),
                                }
                            }
                            _ => {
                                let value = match value.as_str() {
                                    Some(value) => value,
                                    None => panic!(
                                        "Colormap entry value is not a string in {:?}",
                                        manifest_path
                                    ),
                                };
                                entries.insert(key.to_string(), value.to_string());
                            }
                        }
                    }

                    self.colormaps.insert(
                        name,
                        Colormap {
                            label,
                            shortcode,
                            codepoint,
                            description,
                            entries,
                        },
                    );
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

                            full_path.push(match src.as_str() {
                                Some(src) => src,
                                None => panic!("Emoji src is not a string in {:?}", manifest_path),
                            });

                            match full_path.canonicalize() {
                                Ok(full_path) => full_path,
                                Err(err) => panic!("Could not find emoji src file '{:?}' with error '{}' in '{:?}'", full_path, err, manifest_path),
                            }
                        }
                        None => panic!("Emoji is missing 'src' in {:?}", manifest_path),
                    };

                    let name = match emoji.get("name") {
                        Some(name) => match name.as_str() {
                            Some(name) => name.to_string(),
                            None => panic!("Emoji name is not a string in {:?}", manifest_path),
                        },
                        None => panic!("Emoji is missing 'name' in {:?}", manifest_path),
                    };

                    let description = match emoji.get("description") {
                        Some(description) => match description.as_str() {
                            Some(description) => description.to_string(),
                            None => {
                                panic!("Emoji description is not a string in {:?}", manifest_path)
                            }
                        },
                        None => panic!("Emoji is missing 'description' in {:?}", manifest_path),
                    };

                    let category: Vec<String> = match emoji.get("category") {
                        Some(category) => category
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|c| match c.as_str() {
                                Some(c) => c.to_string(),
                                None => {
                                    panic!("Emoji category is not a string in {:?}", manifest_path)
                                }
                            })
                            .collect(),
                        None => panic!("Emoji is missing 'category' in {:?}", manifest_path),
                    };

                    let tags: Vec<String> = match emoji.get("tags") {
                        Some(tags) => tags
                            .as_array()
                            .unwrap()
                            .iter()
                            .map(|t| match t.as_str() {
                                Some(t) => t.to_string(),
                                None => panic!("Emoji tag is not a string in {:?}", manifest_path),
                            })
                            .collect(),
                        None => panic!("Emoji is missing 'tags' in {:?}", manifest_path),
                    };

                    let codepoint: Option<Vec<String>> = match emoji.get("codepoint") {
                        Some(codepoint) => Some(
                            codepoint
                                .as_array()
                                .expect(&format!(
                                    "Emoji 'codepoint' is not an array in {:?}",
                                    manifest_path
                                ))
                                .iter()
                                .map(|c| match c.as_str() {
                                    Some(c) => c.to_string(),
                                    None => panic!(
                                        "Emoji codepoint is not a string in {:?}",
                                        manifest_path
                                    ),
                                })
                                .collect(),
                        ),
                        None => None,
                    };

                    let shortcodes: Vec<String> = match emoji.get("shortcodes") {
                        Some(shortcode) => shortcode
                            .as_array()
                            .expect(&format!(
                                "Emoji 'shortcodes' is not an array in {:?}",
                                manifest_path
                            ))
                            .iter()
                            .map(|s| match s.as_str() {
                                Some(s) => s.to_string(),
                                None => {
                                    panic!("Emoji contains invalid shortcode (must be a string) in {:?}", manifest_path)
                                }
                            })
                            .collect(),
                        None => vec![],
                    };

                    println!("shortcodes: {:?}", shortcodes);

                    let colormaps: Vec<String> = match emoji.get("colormaps") {
                        Some(colormaps) => colormaps
                            .as_array()
                            .expect(&format!(
                                "Emoji 'colormaps' is not an array in {:?}",
                                manifest_path
                            ))
                            .iter()
                            .map(|c| match c.as_str() {
                                Some(c) => c.to_string(),
                                None => {
                                    panic!("Emoji colormap is not a string in {:?}", manifest_path)
                                }
                            })
                            .collect(),
                        None => vec![],
                    };

                    self.emojis.push(Emoji {
                        src,
                        svg: None,
                        name,
                        description,
                        category,
                        tags,
                        codepoint,
                        shortcodes,
                        colormaps,
                        colormap_entries: HashMap::new(),
                    });
                }
            }

            // Target
            if let Some(targets) = manifest.get("target") {
                let targets = targets.as_array().unwrap();

                for target in targets.iter() {
                    let target = target.as_table().unwrap();

                    let name = match target.get("name") {
                        Some(name) => match name.as_str() {
                            Some(name) => name.to_string(),
                            None => panic!("Target name is not a string in {:?}", manifest_path),
                        },
                        None => panic!("Target is missing 'name' in {:?}", manifest_path),
                    };

                    let tags: Vec<String> = match target.get("tags") {
                        Some(tags) => tags
                            .as_array()
                            .expect(&format!(
                                "Target 'tags' is not an array in {:?}",
                                manifest_path
                            ))
                            .iter()
                            .map(|t| match t.as_str() {
                                Some(t) => t.to_string(),
                                None => panic!("Target tag is not a string in {:?}", manifest_path),
                            })
                            .collect(),
                        None => panic!("Target is missing 'tags' in {:?}", manifest_path),
                    };

                    let include_tags: Vec<String> = match target.get("include_tags") {
                        Some(tags) => tags
                            .as_array()
                            .expect(&format!(
                                "Target 'include_tags' is not an array in {:?}",
                                manifest_path
                            ))
                            .iter()
                            .map(|t| match t.as_str() {
                                Some(t) => t.to_string(),
                                None => panic!(
                                    "Target include_tag is not a string in {:?}",
                                    manifest_path
                                ),
                            })
                            .collect(),
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
                                    _ => panic!("Target contains unknown 'structure.container' '{}' in {:?}", container, manifest_path),
                                }
                                _ => panic!("Target is missing 'structure.container' in {:?}", manifest_path),
                            };

                            let filenames = match structure.get("filenames") {
                                Some(filenames) => match filenames.as_str().unwrap() {
                                    "shortcode" => FilenameFormat::Shortcode,
                                    "codepoint" => FilenameFormat::Codepoint,
                                    _ => panic!("Target contains unknown 'structure.filenames' '{}' in {:?}", filenames, manifest_path),
                                }
                                _ => panic!("Target is missing 'structure.filenames' in {:?}", manifest_path),
                            };

                            let subdirectories = match structure.get("subdirectories") {
                                Some(subdirectories) =>
                                    subdirectories
                                    .as_bool()
                                    .expect(&format!("Target contains invalid 'structure.subdirectories' '{}' in {:?}", subdirectories, manifest_path)),
                                None => panic!("Target is missing 'structure.subdirectories' in {:?}", manifest_path),
                            };

                            OutputStructure {
                                container,
                                filenames,
                                subdirectories,
                            }
                        }
                        None => panic!("Target is missing 'structure' in {:?}", manifest_path),
                    };

                    let output_format = match target.get("output") {
                        Some(output) => {
                            let output = output.as_table().unwrap();

                            let size = match output.get("size") {
                                Some(size) => match size.as_integer() {
                                    Some(size) => {
                                        if size < 0 {
                                            panic!("Target contains 'output.size' '{}' (under 0) in {:?}", size, manifest_path);
                                        }

                                        if size > 65536 {
                                            panic!("Target contains 'output.size' '{}' (over 65536) in {:?}", size, manifest_path);
                                        }

                                        Some(size as u32)
                                    }
                                    None => panic!(
                                        "Target contains invalid 'output.size' '{}' in {:?}",
                                        size, manifest_path
                                    ),
                                },
                                None => None,
                            };

                            let compression = match output.get("compression") {
                                Some(compression) => match compression.as_float() {
                                    Some(compression) => {
                                        if compression < 0.0 {
                                            panic!("Target contains 'output.compression' '{}' (under 0.0) in {:?}", compression, manifest_path);
                                        }

                                        Some(compression)
                                    },
                                    None => panic!("Target contains invalid 'output.compression' '{}' (must contain a decimal point) in {:?}", compression, manifest_path),
                                },
                                None => None,
                            };

                            match output.get("format") {
                                Some(format) => match format.as_str().unwrap() {
                                    "none" => OutputFormat::None,
                                    "svg" => OutputFormat::Svg,
                                    "png-image" => OutputFormat::Raster {
                                        format: EncodeTarget::PngImage,
                                        size: size.unwrap() as u32,
                                    },
                                    "png-oxipng-zopfli" => {
                                        match compression {
                                            Some(compression) => {
                                                if compression > 14.0 {
                                                    panic!("Target uses 'png-oxipng-zopfli', but contains 'output.compression' '{}' (must be 0-14) in {:?}", compression, manifest_path);
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
                                                    panic!("Target uses 'png-oxipng-libdeflater', but contains 'output.compression' '{}' (must be 0-12) in {:?}", compression, manifest_path);
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
                                                    panic!("Target uses 'avif-lossy', but contains 'output.compression' '{}' (must be 0.0-100.0) in {:?}", compression, manifest_path);
                                                }

                                                OutputFormat::Raster {
                                                    format: EncodeTarget::Avif { quality: compression as f32, speed: 1 },
                                                    size: size.unwrap() as u32,
                                                }
                                            },
                                            None => panic!("Target uses 'avif-lossy', but doesn't specify 'output.compression' in {:?}", manifest_path),
                                        }
                                    }
                                    _ => panic!("Target contains unknown 'output.format' '{}' in {:?}", format, manifest_path),
                                },
                                None => panic!("Target is missing 'output.format' in {:?}", manifest_path),
                            }
                        }
                        None => panic!("Target is missing 'format' in {:?}", manifest_path),
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
    }
}
