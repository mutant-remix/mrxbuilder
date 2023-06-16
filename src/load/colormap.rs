use crate::load::{Pack, Emoji};

impl Pack {
    pub fn resolve_colormaps(&mut self) {
        let mut new_emojis: Vec<Emoji> = Vec::new();

        for emoji in self.emojis.clone() {
            if emoji.colormaps.len() > 0 {
                for colormap_name in &emoji.colormaps {
                    let mut emoji = emoji.clone();

                    let colormap = match self.colormaps.get(colormap_name) {
                        Some(colormap) => colormap,
                        None => panic!("Emoji '{}' uses colormap '{}' which is undefined", emoji.name, colormap_name),
                    };

                    if emoji.name.contains("%label") {
                        let label = match &colormap.label {
                            Some(label) => label,
                            None => panic!("Emoji '{}' uses %label, but colormap '{}' does not have a label ", emoji.name, colormap_name),
                        };

                        emoji.name = emoji.name.replace("%label", label);
                    }

                    new_emojis.push(emoji);
                }
            } else {
                new_emojis.push(emoji);
            }
        }

        self.emojis = new_emojis;
    }
}
