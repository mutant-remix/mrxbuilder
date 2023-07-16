# Output metadata
The output metadata consists of a JSON file with the structure shown below. It is loosely based on [Google emoji metadata](https://github.com/googlefonts/emoji-metadata).

```json
[
    {
        "group": "group_name",
        "emojis": [
            // Emoji objects
        ]
    }
]
```

## Emoji object
### Required fields
- `src` is the path to the emoji image
- `description` is a string

### Optional fields
- `shortcodes` can be an array of strings, or an empty array
- `codepoint` and `root_codepoint` can be an array of numbers, or an empty array
- `category` can be an array of strings, or an empty array

```json
{
    "codepoint": [
        9996,
        8205,
        128994
    ],
    "root_codepoint": [
        9996
    ],
    "src": "expressions/skintones/human/victory_hand_g",
    "shortcodes": [
        ":victory_hand_g:",
        ":v_g:"
    ],
    "category": [
        "expressions",
        "skintones",
        "human"
    ],
    "description": "something"
},
```
