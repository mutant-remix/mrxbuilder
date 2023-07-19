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
    src: Option<String>,
    base: Option<Vec<u64>>,
    alternates: Vec<Vec<u64>>,
    shortcodes: Vec<String>,
    category: Vec<String>,
    description: String,
    emoticons: Vec<String>,
    animated: bool,
}

fn parse_codepoint(codepoint: &Vec<String>) -> Vec<u64> {
    codepoint
        .iter()
        .map(|codepoint| {
            let codepoint = codepoint.replace("U+", "");
            u64::from_str_radix(&codepoint, 16).unwrap()
        })
        .collect()
}

pub fn generate_metadata(emojis: &Vec<EmojiEncoded>) -> String {
    let mut alternate_map: HashMap<Vec<u64>, Vec<Vec<u64>>> = HashMap::new();
    for emoji in emojis {
        let root_codepoint = match &emoji.emoji.root_codepoint {
            Some(codepoint) => parse_codepoint(codepoint),
            None => continue,
        };

        let codepoint = match &emoji.emoji.codepoint {
            Some(codepoint) => parse_codepoint(codepoint),
            None => continue,
        };

        if codepoint == root_codepoint {
            continue;
        }

        if !alternate_map.contains_key(&root_codepoint) {
            alternate_map.insert(root_codepoint.to_owned(), Vec::new());
        }

        let alternates = alternate_map.get_mut(&root_codepoint).unwrap();
        if !alternates.contains(&codepoint) {
            alternates.push(codepoint);
        }
    }

    let mut groups: HashMap<String, Vec<Emoji>> = HashMap::new();
    for emoji in emojis {
        let group = emoji.emoji.category[0].clone();
        if !groups.contains_key(&group) {
            groups.insert(group.clone(), Vec::new());
        }

        let codepoint: Option<Vec<u64>> = match &emoji.emoji.codepoint {
            Some(codepoint) => Some(parse_codepoint(codepoint)),
            None => None,
        };

        let shortcodes = emoji
            .emoji
            .shortcodes
            .iter()
            .map(|s| format!(":{}:", s))
            .collect();

        let alternates = match &codepoint {
            Some(codepoint) => match alternate_map.get(codepoint) {
                Some(alternates) => alternates.to_owned(),
                None => Vec::with_capacity(0),
            },
            None => Vec::with_capacity(0),
        };

        let emoji = Emoji {
            src: emoji.filename.clone(),
            base: codepoint,
            alternates,
            category: emoji.emoji.category.clone(),
            shortcodes,
            description: emoji.emoji.description.clone(),
            emoticons: Vec::with_capacity(0),
            animated: false,
        };

        groups.get_mut(&group).unwrap().push(emoji);
    }

    let mut final_groups: Vec<Group> = Vec::new();
    for (group, emojis) in groups {
        final_groups.push(Group { group, emojis });
    }

    serde_json::to_string_pretty(&final_groups).unwrap()
}
