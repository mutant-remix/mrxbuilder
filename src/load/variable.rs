use crate::load::Pack;

impl Pack {
    pub fn resolve_variables(&mut self) {
        // Colormaps
        for (_, colormap) in &mut self.colormaps {
            for (key, value) in &colormap.entries.clone() {
                if key.starts_with("$") {
                    match self.definitions.get(key) {
                        Some(variable) => {
                            colormap.entries.remove(key);
                            colormap.entries.insert(variable.clone(), value.clone());
                        },
                        None => panic!("Colormap uses variable '{}' in key, which is undefined", key),
                    }
                }
            }

            for (key, value) in &colormap.entries.clone() {
                if value.starts_with("$") {
                    match self.definitions.get(value) {
                        Some(variable) => {
                            colormap.entries.remove(key);
                            colormap.entries.insert(key.clone(), variable.clone());
                        },
                        None => panic!("Colormap uses variable '{}' in value, which is undefined", key),
                    }
                }
            }

            if let Some(codepoint) = &mut colormap.codepoint {
                if codepoint.starts_with("$") {
                    match self.definitions.get(codepoint) {
                        Some(variable) => {
                            *codepoint = variable.clone();
                        },
                        None => panic!("Colormap uses variable '{}' in codepoint, which is undefined", codepoint),
                    }
                }
            };
        }

        // Emojis
        for emoji in &mut self.emojis {
            if let Some(codepoint) = &mut emoji.codepoint {
                let mut new_codepoint: Vec<String> = Vec::new();

                for codepoint_component in &codepoint.clone() {
                    if codepoint_component.starts_with("$") {
                        match self.definitions.get(codepoint_component) {
                            Some(variable) => {
                                for variable_component in variable.split(" ") {
                                    new_codepoint.push(variable_component.to_string());
                                }
                            },
                            None => panic!("Emoji uses variable '{}' in codepoint, which is undefined", codepoint_component),
                        }
                    } else {
                        new_codepoint.push(codepoint_component.clone());
                    }
                }

                *codepoint = new_codepoint;
            };

            let mut new_colormaps: Vec<String> = Vec::new();

            for colormap in &mut emoji.colormaps {
                if colormap.starts_with("$") {
                    match self.definitions.get(colormap) {
                        Some(variable) => {
                            for variable_component in variable.split(" ") {
                                new_colormaps.push(variable_component.to_string());
                            }
                        },
                        None => panic!("Emoji uses variable '{}' in colormap, which is undefined", colormap),
                    }
                } else {
                    new_colormaps.push(colormap.clone());
                }
            }

            emoji.colormaps = new_colormaps;
        }
    }
}
