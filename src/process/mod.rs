use rayon::prelude::*;
use std::fs;

pub mod encode;
use encode::encode_raster;

mod rasterize;
use rasterize::rasterise_svg;

pub mod cache;

mod package;
use package::Package;

use crate::load::manifest::{OutputFormat, FilenameFormat, Emoji};
use crate::Pack;

struct EmojiEncoded {
    emoji: Emoji,
    raster: Option<Vec<u8>>,
    filename: Option<String>,
}

impl Pack {
    pub fn build_tags(&mut self, tags: Vec<String>) {
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
        self.logger.set_stage_count(self.targets.len() * 2 + 1);

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
                let svg = &emoji.emoji.svg.as_ref().unwrap().0;

                let encoded = match &target.output_format {
                    OutputFormat::Raster { format, size } => {
                        match self.cache.try_get(svg, &format, *size) {
                            Some(encoded) => Some(encoded),
                            None => {
                                let raster = rasterise_svg(svg, *size);
                                let encoded = encode_raster(&raster, &format);

                                self.cache.save(
                                    &svg,
                                    &format,
                                    *size,
                                    &encoded,
                                );

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

                stage.clone().inc();

                emoji.raster = encoded;
                emoji.filename = Some(filename);
            });

            // Generate metadata
            for emoji in &emojis {
                // TODO
            }

            let mut stage = self.logger.new_stage("Writing", emojis.len() + target.include_files.len());

            let path = self.output_path.join(&target.name);
            let mut package = Package::new(&target.output_structure.container, &path);

            // Write emojis
            for emoji in &emojis {
                match &target.output_format {
                    OutputFormat::None => {},
                    OutputFormat::Svg => {
                        let mut filename = emoji.filename.as_ref().unwrap().clone();
                        filename.push_str(".svg");

                        package.add_file(
                            &emoji.emoji.svg.as_ref().unwrap().0.as_bytes().to_vec(),
                            &filename
                        );
                    }
                    OutputFormat::Raster { format, size: _ } => {
                        let mut filename = emoji.filename.as_ref().unwrap().clone();
                        let extension = format.to_extension();
                        filename.push_str(&format!(".{}", extension));

                        package.add_file(
                            emoji.raster.as_ref().unwrap(),
                            &filename
                        );
                    }
                }

                stage.inc();
            }

            // Write metadata
            // TODO

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

                stage.inc();
            }

            package.finish();
        }
    }
}
