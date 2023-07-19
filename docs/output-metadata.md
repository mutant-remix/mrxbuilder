# Output metadata
The output metadata consists of a JSON file with the structure shown below.

It is designed to be compatible with [Google emoji metadata](https://github.com/googlefonts/emoji-metadata).

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
Name | Type | Notes
--- | --- | ---
`src` | `string` | relative path to the emoji image
`shortcodes` | `string[]` | _can be empty_
`base` | `number[] \| null` | codepoint of this emoji, if it has one
`alternates` | `number[][]` | codepoints of emojis that are variations of this emoji
`category` | `string[]` | the category of this emoji
`description` | `string` |
`emoticons` | `string[]` | **Always** empty array
`animated` | `boolean` | **Always** false

```json
{
    "src": "expressions/skintones/human/victory_hand",
    "base": [
        9996
    ],
    "alternates": [
        [
            9996,
            8205,
            128997
        ],
        [
            9996,
            8205,
            128994
        ]
    ],
    "shortcodes": [
        ":victory_hand:",
        ":v:"
    ],
    "category": [
        "expressions",
        "skintones",
        "human"
    ],
    "description": "something",
    "emoticons": [],
    "animated": false
}
```
