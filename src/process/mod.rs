use rayon::prelude::*;
use std::collections::HashMap;

pub mod encode;
use encode::encode_raster;

mod rasterize;
use rasterize::rasterise_svg;

pub mod cache;

use crate::load::manifest::{OutputFormat, FilenameFormat};
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
        self.logger.set_stage_count(self.targets.len() + 1);

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
                        let svg = &emoji.svg.as_ref().unwrap().0;

                        stage.clone().inc();

                        match self.cache.try_get(svg, encode_target, *size) {
                            Some(encoded) => (emoji.name.clone(), encoded),
                            None => {
                                let raster = rasterise_svg(svg, *size);
                                let encoded = encode_raster(&raster, encode_target);

                                self.cache.save(
                                    &emoji.svg.as_ref().unwrap().0,
                                    encode_target,
                                    *size,
                                    &encoded,
                                );

                                let filename = match &target.output_structure.filenames {
                                    FilenameFormat::Codepoint => match emoji.to_codepoint_filename() {
                                        Some(filename) => filename,
                                        None => {
                                            panic!("Target '{}' requires codepoint filename for emoji '{}', but it does not have a codepoint", target.name, emoji.name);
                                        }
                                    },
                                    FilenameFormat::Shortcode => match emoji.to_shortcode_filename() {
                                        Some(filename) => filename,
                                        None => {
                                            panic!("Target '{}' requires shortcode filename for emoji '{}', but it does not have a shortcode", target.name, emoji.name);
                                        }
                                    },
                                };

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
                            FilenameFormat::Shortcode => match emoji.to_shortcode_filename() {
                                Some(filename) => filename,
                                None => {
                                    panic!("Target '{}' requires shortcode filename for emoji '{}', but it does not have a shortcode", target.name, emoji.name);
                                }
                            },
                        };

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

            // write, including extra files
        }
    }
}
