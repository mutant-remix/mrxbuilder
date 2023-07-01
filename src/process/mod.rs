use rayon::prelude::*;

pub mod encode;
use encode::encode_raster;

mod rasterize;
use rasterize::rasterise_svg;

pub mod cache;

use crate::Pack;
use crate::load::manifest::{OutputFormat};

impl Pack {
    pub fn build_tag(&mut self, tag: &str) {
        let targets = self
            .targets
            .iter()
            .filter(|target| target.tags.contains(&tag.to_string()))
            .collect::<Vec<_>>();

        self.logger.info(&format!("Selected {} targets tagged '{}'", targets.len(), tag));
        self.logger.set_stage_count(self.targets.len() + 1);

        for target in targets {
            self.logger.build(&format!("Building target '{}'", target.name));

            let emojis = self
                .emojis
                .iter()
                .filter(|emoji| {
                    for tag in target.include_tags.iter() {
                        if emoji.tags.contains(tag) { return true; }
                    }

                    return false;
                })
                .collect::<Vec<_>>();

            self.logger.build(&format!("Selected {} emojis for target '{}'", emojis.len(), target.name));
            let stage = self.logger.new_stage("Encoding", emojis.len());

            match &target.output_format {
                OutputFormat::Raster { format: encode_target, size } => {
                    let encoded: Vec<Vec<u8>> = emojis.par_iter().map(|emoji| {
                        let svg = &emoji.svg.as_ref().unwrap().0;

                        stage.clone().inc();

                        match self.cache.try_get(svg, encode_target, *size) {
                            Some(encoded) => encoded,
                            None => {
                                let raster = rasterise_svg(svg, *size);
                                let encoded = encode_raster(&raster, *size, encode_target);

                                self.cache.save(&emoji.svg.as_ref().unwrap().0, encode_target, *size, &raster.as_raw());

                                encoded
                            }
                        }
                    }).collect();
                }
                OutputFormat::Svg => {

                }
                OutputFormat::None => {}
            };

            // todo: metadata

            // include extra files

            // write to container
        }
    }
}
