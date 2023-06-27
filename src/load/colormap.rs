use crate::load::{Emoji, Pack};
use resvg::usvg::Color;

fn parse_hex_str(hex_str: &str) -> Color {
    let hex_str = hex_str.trim_start_matches("#");

    let r = u8::from_str_radix(&hex_str[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex_str[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex_str[4..6], 16).unwrap();

    Color {
        red: r,
        green: g,
        blue: b,
    }
}

impl Pack {
    pub fn resolve_colormaps(&mut self) {
        let mut new_emojis: Vec<Emoji> = Vec::new();

        for emoji in self.emojis.clone() {
            if emoji.colormaps.len() > 0 {
                for colormap_name in &emoji.colormaps {
                    let mut emoji = emoji.clone();

                    let colormap = match self.colormaps.get(colormap_name) {
                        Some(colormap) => colormap,
                        None => panic!(
                            "Emoji '{}' uses colormap '{}' which is undefined",
                            emoji.name, colormap_name
                        ),
                    };

                    if emoji.name.contains("%label") {
                        let label = match &colormap.label {
                            Some(label) => label,
                            None => panic!(
                                "Emoji '{}' uses %label, but colormap '{}' does not have a label ",
                                emoji.name, colormap_name
                            ),
                        };

                        emoji.name = emoji.name.replace("%label", label);
                    }

                    for shortcode in &mut emoji.shortcodes {
                        if shortcode.contains("%shortcode") {
                            let colormap_shortcode = match &colormap.shortcode {
                                Some(colormap_shortcode) => colormap_shortcode,
                                None => panic!("Emoji '{}' uses %shortcode, but colormap '{}' does not have a shortcode ", emoji.name, colormap_name),
                            };

                            *shortcode = shortcode.replace("%shortcode", colormap_shortcode);
                        }
                    }

                    if let Some(codepoint) = &mut emoji.codepoint {
                        for codepoint in codepoint.iter_mut() {
                            if codepoint == "%codepoint" {
                                let colormap_codepoint = match &colormap.codepoint {
                                    Some(colormap_codepoint) => colormap_codepoint,
                                    None => panic!("Emoji '{}' uses %codepoint, but colormap '{}' does not have a codepoint ", emoji.name, colormap_name),
                                };

                                *codepoint = colormap_codepoint.clone().to_string();
                            }
                        }
                    }

                    if emoji.description.contains("%description") {
                        let colormap_description = match &colormap.description {
                            Some(colormap_description) => colormap_description,
                            None => panic!("Emoji '{}' uses %description, but colormap '{}' does not have a description ", emoji.name, colormap_name),
                        };

                        emoji.description = emoji
                            .description
                            .replace("%description", colormap_description);
                    }

                    let mut colormap_entries: Vec<(Color, Color)> = Vec::new();
                    for (key, value) in &colormap.entries {
                        if !key.starts_with("#") {
                            panic!(
                                "Colormap '{}' has an invalid source color '{}'",
                                colormap_name, key
                            );
                        }

                        if !value.starts_with("#") {
                            panic!(
                                "Colormap '{}' has an invalid target color '{}'",
                                colormap_name, value
                            );
                        }

                        let source = parse_hex_str(&key);
                        let target = parse_hex_str(&value);

                        colormap_entries.push((source, target));
                    }

                    emoji.colormaps.clear();

                    let mut svg = emoji.svg.take().unwrap();
                    svg.replace_colors(colormap_entries);
                    emoji.svg = Some(svg);

                    new_emojis.push(emoji);
                }
            } else {
                new_emojis.push(emoji);
            }
        }

        self.emojis = new_emojis;
    }
}
