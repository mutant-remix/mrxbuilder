use std::{fs, thread};
use rayon::prelude::*;

pub mod encode;
use encode::encode_raster;

mod rasterize;
use rasterize::rasterise_svg;

pub mod cache;

mod metadata;
use metadata::generate_metadata;

mod package;
use package::Package;

use crate::load::manifest::{OutputFormat, FilenameFormat, Emoji};
use crate::Pack;

pub struct EmojiEncoded {
    pub filename: Option<String>,
    pub emoji: Emoji,
    raster: Option<Vec<u8>>,
}

impl Pack {
    pub fn build_tags(&mut self, tags: Vec<String>, dry: bool) {
        let targets = self
            .targets
            .iter()
            .filter(|target| {
                for tag in tags.iter() {
                    if target.tags.contains(tag) {
                        return true;
                    }
                }

                return false;
            })
            .collect::<Vec<_>>();

        self.logger.info(&format!(
            "Selected {} targets tagged '{}'",
            targets.len(),
            tags.join(", ")
        ));
        self.logger.set_stage_count(self.targets.len() + 1);

        for target in targets {
            self.logger
                .build(&format!("Building target '{}'", target.name));

            let mut emojis: Vec<EmojiEncoded> = self
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
                .map(|emoji| EmojiEncoded {
                    emoji: emoji.clone(),
                    raster: None,
                    filename: None,
                })
                .collect::<Vec<_>>();

            self.logger.build(&format!(
                "Selected {} emojis for target '{}'",
                emojis.len(),
                target.name
            ));
            let stage = self.logger.new_stage("Encoding", emojis.len());

            // Encode and generate filenames
            emojis.par_iter_mut().for_each(|emoji| {
                stage.clone().inc();

                let svg = &emoji.emoji.svg.as_ref().unwrap().0;

                let encoded = match &target.output_format {
                    OutputFormat::Raster { format, size } => {
                        match self.cache.try_get(svg, &format, *size) {
                            Some(encoded) => Some(encoded),
                            None => {
                                let raster = rasterise_svg(svg, *size);
                                let encoded = encode_raster(&raster, &format);

                                if !dry {
                                    self.cache.save(
                                        &svg,
                                        &format,
                                        *size,
                                        &encoded,
                                    );
                                };

                                Some(encoded)
                            }
                        }
                    },
                    OutputFormat::Svg => None,
                    OutputFormat::None => None,
                };

                let filename = match &target.output_structure.filenames {
                    FilenameFormat::Codepoint => match emoji.emoji.to_codepoint_filename(target.output_structure.flat) {
                        Some(filename) => filename,
                        None => {
                            panic!("Target '{}' requires codepoint filename for emoji '{}', but it does not have a codepoint", target.name, emoji.emoji.name);
                        }
                    },
                    FilenameFormat::Shortcode => match emoji.emoji.to_shortcode_filename(target.output_structure.flat) {
                        Some(filename) => filename,
                        None => {
                            panic!("Target '{}' requires shortcode filename for emoji '{}', but it does not have a shortcode", target.name, emoji.emoji.name);
                        }
                    },
                };

                let filename = match &target.output_format {
                    OutputFormat::Svg => format!("{}.svg", filename),
                    OutputFormat::Raster { format, size: _ } => {
                        let extension = format.to_extension();
                        format!("{}.{}", filename, extension)
                    },
                    OutputFormat::None => filename,
                };

                emoji.raster = encoded;
                emoji.filename = Some(filename);
            });

            // Save on a separate thread
            // To continue encoding while saving
            let path = self.output_path.join(&target.name);
            let target = target.clone();

            if let Some(save_thread) = self.save_thread.take() {
                save_thread.join().unwrap();
            }

            let save_thread = thread::spawn(move || {
                // Generate metadata
                let mut categories = Vec::new();
                for emoji in &emojis {
                    let category = emoji.emoji.category[0].clone();

                    if !categories.contains(&category) {
                        categories.push(category);
                    }
                }

                let mut package = Package::new(&target.output_structure.container, &path, dry);

                // Write emojis
                for emoji in &emojis {
                    match &target.output_format {
                        OutputFormat::None => {},
                        OutputFormat::Svg => {
                            package.add_file(
                                &emoji.emoji.svg.as_ref().unwrap().0.as_bytes().to_vec(),
                                emoji.filename.as_ref().unwrap()
                            );
                        }
                        OutputFormat::Raster { format: _, size: _ } => {
                            package.add_file(
                                emoji.raster.as_ref().unwrap(),
                                emoji.filename.as_ref().unwrap()
                            );
                        }
                    }
                }

                // Write metadata
                let metadata = generate_metadata(&emojis);
                package.add_file(&metadata.as_bytes().to_vec(), "metadata.json");

                // Write extra files
                for file in target.include_files.iter() {
                    let filename = match file.file_name() {
                        Some(filename) => filename.to_str().unwrap(),
                        None => {
                            panic!("Failed to get filename for file '{}' while building target '{}'", file.display(), target.name);
                        }
                    };

                    match fs::read(file) {
                        Ok(file) => {
                            package.add_file(&file, filename);
                        },
                        Err(error) => {
                            panic!("Failed to read file '{}' while building target '{}': {}", file.display(), target.name, error);
                        }
                    };
                }

                package.finish();
            });

            self.save_thread = Some(save_thread);
        }
    }
}
