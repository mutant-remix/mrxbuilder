use serde::Serialize;
use std::collections::HashMap;

use crate::process::EmojiEncoded;

#[derive(Serialize)]
struct Group {
    group: String,
    emojis: Vec<Emoji>,
}

#[derive(Serialize)]
struct Emoji {
    codepoint: Option<Vec<u64>>,
    root_codepoint: Option<Vec<u64>>,
    src: Option<String>,
    shortcodes: Vec<String>,
    category: Vec<String>,
    description: String,
}

pub fn generate_metadata(emojis: &Vec<EmojiEncoded>) -> String {
    let mut groups: HashMap<String, Vec<Emoji>> = HashMap::new();

    for emoji in emojis {
        let group = emoji.emoji.category[0].clone();
        if !groups.contains_key(&group) {
            groups.insert(group.clone(), Vec::new());
        }

        let codepoint: Option<Vec<u64>> = match emoji.emoji.codepoint.as_ref() {
            Some(codepoint) => Some(
                codepoint
                    .iter()
                    .map(|codepoint| {
                        let codepoint = codepoint.replace("U+", "");
                        u64::from_str_radix(&codepoint, 16).unwrap()
                    })
                    .collect(),
            ),
            None => None,
        };

        let root_codepoint: Option<Vec<u64>> = match emoji.emoji.root_codepoint.as_ref() {
            Some(codepoint) => Some(
                codepoint
                    .iter()
                    .map(|codepoint| {
                        let codepoint = codepoint.replace("U+", "");
                        u64::from_str_radix(&codepoint, 16).unwrap()
                    })
                    .collect(),
            ),
            None => None,
        };

        let shortcodes = emoji
            .emoji
            .shortcodes
            .iter()
            .map(|s| format!(":{}:", s))
            .collect();

        let emoji = Emoji {
            codepoint,
            root_codepoint: root_codepoint,
            src: emoji.filename.clone(),
            category: emoji.emoji.category.clone(),
            shortcodes,
            description: emoji.emoji.description.clone(),
        };

        groups.get_mut(&group).unwrap().push(emoji);
    }

    let mut final_groups: Vec<Group> = Vec::new();
    for (group, emojis) in groups {
        final_groups.push(Group { group, emojis });
    }

    serde_json::to_string_pretty(&final_groups).unwrap()
}
