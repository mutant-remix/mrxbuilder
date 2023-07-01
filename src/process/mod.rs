use rayon::prelude::*;
use std::{collections::HashMap, fs};

pub mod encode;
use encode::encode_raster;

mod rasterize;
use rasterize::rasterise_svg;

pub mod cache;

use crate::load::manifest::{OutputFormat, FilenameFormat, Container};
use crate::Pack;

impl Pack {
    pub fn build_tag(&mut self, tag: &str) {
        let targets = self
            .targets
            .iter()
            .filter(|target| target.tags.contains(&tag.to_string()))
            .collect::<Vec<_>>();

        self.logger.info(&format!(
            "Selected {} targets tagged '{}'",
            targets.len(),
            tag
        ));
        self.logger.set_stage_count(self.targets.len() * 2 + 1);

        for target in targets {
            self.logger
                .build(&format!("Building target '{}'", target.name));

            let emojis = self
                .emojis
                .iter()
                .filter(|emoji| {
                    for tag in target.include_tags.iter() {
                        if emoji.tags.contains(tag) {
                            return true;
                        }
                    }

                    return false;
                })
                .collect::<Vec<_>>();

            self.logger.build(&format!(
                "Selected {} emojis for target '{}'",
                emojis.len(),
                target.name
            ));
            let stage = self.logger.new_stage("Encoding", emojis.len());

            // TODO: Generate metadata

            // Encode
            let files = match &target.output_format {
                OutputFormat::Raster {
                    format: encode_target,
                    size,
                } => {
                    let files = emojis.par_iter().map(|emoji| {
                        stage.clone().inc();

                        let svg = &emoji.svg.as_ref().unwrap().0;

                        let filename = match &target.output_structure.filenames {
                            FilenameFormat::Codepoint => match emoji.to_codepoint_filename() {
                                Some(filename) => filename,
                                None => {
                                    panic!("Target '{}' requires codepoint filename for emoji '{}', but it does not have a codepoint", target.name, emoji.name);
                                }
                            },
                            FilenameFormat::Shortcode => match emoji.to_shortcode_filename(target.output_structure.subdirectories) {
                                Some(filename) => filename,
                                None => {
                                    panic!("Target '{}' requires shortcode filename for emoji '{}', but it does not have a shortcode", target.name, emoji.name);
                                }
                            },
                        };
                        let filename = format!("{}.{}", filename, encode_target.to_extension());

                        match self.cache.try_get(svg, encode_target, *size) {
                            Some(encoded) => (filename, encoded),
                            None => {
                                let raster = rasterise_svg(svg, *size);
                                let encoded = encode_raster(&raster, encode_target);

                                self.cache.save(
                                    &emoji.svg.as_ref().unwrap().0,
                                    encode_target,
                                    *size,
                                    &encoded,
                                );

                                (filename, encoded)
                            }
                        }
                    });

                    let files: HashMap<String, Vec<u8>> = files.collect();

                    files
                }
                OutputFormat::Svg => {
                    let files = emojis.iter().map(|emoji| {
                        let filename = match &target.output_structure.filenames {
                            FilenameFormat::Codepoint => match emoji.to_codepoint_filename() {
                                Some(filename) => filename,
                                None => {
                                    panic!("Target '{}' requires codepoint filename for emoji '{}', but it does not have a codepoint", target.name, emoji.name);
                                }
                            },
                            FilenameFormat::Shortcode => match emoji.to_shortcode_filename(target.output_structure.subdirectories) {
                                Some(filename) => filename,
                                None => {
                                    panic!("Target '{}' requires shortcode filename for emoji '{}', but it does not have a shortcode", target.name, emoji.name);
                                }
                            },
                        };
                        let filename = format!("{}.svg", filename);

                        (
                            filename,
                            emoji.svg.as_ref().unwrap().0.as_bytes().to_vec(),
                        )
                    });
                    let files: HashMap<String, Vec<u8>> = files.collect();

                    files
                }
                OutputFormat::None => HashMap::new(),
            };

            let stage = self.logger.new_stage("Writing", emojis.len() + target.include_files.len());

            match &target.output_structure.container {
                Container::Directory => {
                    let path = self.output_path.join(&target.name);

                    match fs::remove_dir_all(&path) {
                        Ok(_) => {}
                        Err(err) => {
                            if err.kind() != std::io::ErrorKind::NotFound {
                                panic!("Failed to remove old directory '{}' while building target '{}': {}", path.display(), target.name, err);
                            }
                        }
                    };

                    for (filename, data) in files {
                        let path = path.join(&filename);
                        let dir = path.parent().unwrap();

                        match fs::create_dir_all(dir) {
                            Ok(_) => {}
                            Err(err) => {
                                panic!("Failed to create directory '{}' while building target '{}': {}", dir.display(), target.name, err);
                            }
                        };

                        match fs::write(path, data) {
                            Ok(_) => {}
                            Err(err) => {
                                panic!("Failed to write file '{}' while building target '{}': {}", filename, target.name, err);
                            }
                        };

                        stage.clone().inc();
                    }

                    for file in target.include_files.iter() {
                        let filename = match file.file_name() {
                            Some(filename) => filename.to_str().unwrap(),
                            None => {
                                panic!("Failed to get filename for file '{}' while building target '{}'", file.display(), target.name);
                            }
                        };

                        let path = path.join(filename);

                        match fs::copy(file, path) {
                            Ok(_) => {}
                            Err(err) => {
                                panic!("Failed to copy file '{}' while building target '{}': {}", filename, target.name, err);
                            }
                        };

                        stage.clone().inc();
                    }

                    // TODO: metadata
                }
                Container::Zip => {
                    // TODO
                    unimplemented!();
                }
            }
        }
    }
}
