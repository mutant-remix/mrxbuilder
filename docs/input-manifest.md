# Manifest
This is the specification for the builder's input manifest files, which also acts as the documentation for writing them. It is example-based and hopefully self-explanatory in most cases.

## Overview
The builder's manifests are written in [toml](https://toml.io).

There are only a few entry types:
- [Pack](#pack) - **WIP**
- [Include](#include) - Loads other manifest files
- [Target](#target) - Defines various outputs to build
- [Define](#define) - Defines variables for use in other parts of the manifest
- [Colormap](#colormap) - Defines a colormap to be used in emojis to avoid most repetition
- [Emoji](#emoji) - Defines an emoji

## References
`name`, `shortcode` and `codepoint` are reserved and may not be defined as variable names.

- `#<abcdef>` - RGB hex color
- `U+<1234>` - Unicode codepoint
- `$<name>` - Variable name
- `%` - Colormaps:
    - `%<name>` - Colormap name
    - `%shortcode` exactly - Colormap shortcode (for example, `circle%shortcode`)
    - `%codepoint` - Colormap codepoint (for example, skin tone modifiers)

> `<>` denotes a user-defined value, otherwise it is a literal (don't include the `<>`)

## Entry types
### Pack
**WIP**

```toml
[pack]
```
### Include
The builder only directly loads a single manifest file, typically called `index.toml`. However, it can load other manifest files using `[include]` with the path to other manifest files, relative to the current manifest file. For example:

```toml
[[include]]
path = "./other/manifest.toml"

[[include]]
path = "./another/manifest.toml"
```

### Target
- `output_format`:
    - `format`:
        - Vector images: `svg` - skips rasterization
        - Raster images (must also include width in pixels):
            - `png` - intended to only be used for debug builds, as it produces comparably large files (several times faster)
            - `pngc` - png, but compressed using oxipng
            - `webp` - extremely **fast**, slightly better compression than `pngc`
            - `avif-lossless` - extremely **slow**, slightly better compression than `webp`
    - `container`:
        - `tar.gz`
        - `directory`
        - `structure`:
            - `flat` - true / false - whether to put all emojis in the same directory, or in subdirectories based on their category
            - `filename`
                - `shortcode`
                - `codepoint`
- `tags`: Used when calling the builder to select which targets to build.

```toml
[[target]]
name = "full-shortcode-png-128"
tags = [ "release" ]
include_tags = [ "unicode", "extra" ]
output_format = {
    container = "tar.gz",
    format = [ "png", 128 ],
    structure = {
        flat = true,
        filename = "shortcode",
    },
}

[[target]]
name = "unicode-codepoint-svg"
tags = [ "debug", "release" ]
include_tags = [ "unicode" ]
output_format = {
    container = "directory",
    format = [ "svg" ],
    structure = {
        flat = false,
        filename = "codepoint",
    },
}
```

### Define
Used to define variables for use in other parts of the manifest. The name of the variable will be matched with every instance of `$name` in the manifest (only in values, not keys)

This is also used for palette definitions.

The `name` of the variable must start with `$`.

```toml
# Often repeated codepoints
[[define]]
"$zwj" = "U+200D" # zero-width joiner
"$vs16" = "U+FE0F" # variation selector 16

# Reusable palettes
[[define]]
"$skin_tone.l1" = "#123456"
"$skin_tone.l2" = "#123456"
"$skin_tone.l3" = "#123456"

# Merge multiple colormaps
[[define]]
"$skin_tone.all" = "%skin_tone.l1 %skin_tone.l2 %skin_tone.l3"
```

### Colormap
`shortcode` and `codepoint` are only required when an emoji uses those variables

The `name` of the colormap must start with `%`.

```toml
# Skin tone modifier
[[colormap]]
name = "%skin_tone.l1"
shortcode = "_l1" # light skin tone
codepoint = "U+1F3FB"
"$base-1" = "$skin_tone.l1"

# 3-color flag
[[colormap]]
name = "%flag_lt"
"$base-1" = "#FDBA0B"
"$base-2" = "#006A42"
"$base-3" = "#C22229"
```

### Emoji
- `name`, `description`, `category` - used for metadata
- `labels`: used for metadata
    - `codepoint`: single codepoint split into parts
    - `shortcodes`: list of shortcodes
- `tags`: used in targets to select which emojis to include in that target
- `src`: path to the svg file, relative to the manifest file
- `colormaps` create multiple emoji entries, one for each colormap. `%shortcode` and `%codepoint` will be replaced with the colormap's shortcode and codepoint respectively, and the svg will be recolored with the colormap's entries

> If emojis have overlapping tags, they can't have overlapping names and labels

```toml
# Face
[[emoji]]
name = "Grinning face"
category = "expressions"
description = "A smiling face with smiling eyes and open mouth."
src = "./grinning-face.svg"
tags = [ "unicode" ]
labels = {
    codepoint = [ "U+1F600" ],
    shortcodes = [ "grinning" ]
}

# Non-unicode "extra" emoji with multiple color variations, with a private use area codepoint
[[emoji]]
name = "Human eating a carrot"
category = "activities"
description = "A human eating a carrot."
src = "./eating-carrot.svg"
tags = [ "extra" ]
labels = {
    codepoint = [ "$pua", "U+1234", "%codepoint" ],
    shortcodes = [ "human_eating_carrot%shortcode" ]
}
colormaps = "$skin_tone.all %skin_tone.l4" # notice the $ and % distinction

# Example 3 color flag using a template svg, colored with a colormap
[[emoji]]
name = "Lithuania"
category = "flags"
description = "The flag of Lithuania."
src = "./base_flags/3_equal_horizontal_stripes.svg"
tags = [ "unicode" ]
labels = {
    codepoint = [ "U+1F1F1", "U+1F1F9" ],
    shortcodes = [ "flag_lt", "flag_lithuania", "lithuania" ]
}
colormaps = "%flag_lt"
```
