# Manifest
This is the specification for the builder's input manifest files, which also acts as the documentation for writing them. It is example-based and hopefully self-explanatory in most cases.

## Examples
You can find examples utilising every feature in [sample-input](/sample-input) and below.

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
`name`, `label`, `shortcode` and `codepoint` are reserved and may not be defined as variable names.

- `#<abcdef>` - RGB hex color
- `U+<1234>` - Unicode codepoint
- `$<name>` - Variable name
- `%` - Colormaps:
    - `%<name>` - Colormap name
    - `%label` - Colormap label (for example, skin tone modifiers)
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
paths = [
    "./other/manifest.toml",
    "./another/manifest.toml",
]
```

### Target
- `tags`: Used when calling the builder to select which targets to build.
- `output`:
    - `container`
        - `tar.gz`
        - `zip`
        - `directory`
    - `format`:
        - No images: `none` - used for metadata-only targets
        - Vector images: `svg` - skips rasterization
        - Raster images:
            Format name | Compression levels | Size | Compatibility | Notes
            --- | --- | --- | --- | ---
            `png-image` | n/a | Huge | Best | Intended for quick builds
            `png-oxipng-zopfli` | 0.0-15.0 | Tiny | Best | **recommended for very low resolutions**
            `png-oxipng-libdeflater` | 0.0-12.0 | Small | Best | **recommended**
            `webp` | n/a | Small | Modern browsers | **recommended**
            `avif-lossy` | 100.0-0.0 | Small | Bad | At high quality levels, it is not perceptibly lossy
    - `size` (number) - only for raster images
    - `compression` (number) - for applicable formats
- `structure`
    - `subdirectories` (bool)
    - `filenames`
        - `shortcode`
        - `codepoint`

```toml
[[target]]
name = "full-shortcode-png-128"
tags = [ "release" ]
include_tags = [ "unicode", "extra" ]
output = { format = "png-oxipng-libdeflater", size = 128, compression = 12.0 }
structure = { container = "tar.gz", subdirectories = true, filenames = "shortcode" }

[[target]]
name = "unicode-codepoint-svg"
tags = [ "debug", "release" ]
include_tags = [ "unicode" ]
output = { format = "svg" }
structure = { container = "directory", subdirectories = false, filenames = "codepoint" }

[[target]]
name = "full-metadata"
tags = [ "metadata" ]
include_tags = [ "unicode", "extra" ]
output = { format = "none" }
structure = { container = "directory", subdirectories = false, filenames = "shortcode" }
```

### Define
Used to define variables for use in other parts of the manifest. The name of the variable will be matched with every instance of `$name` in the manifest (only in values, not keys)

This is also used for palette definitions.

The `name` of the variable must start with `$`.

`$` variables will only get resolved in:
- `colormap` entry keys and values
- `colormap.codeoint`
- `emoji.codepoint`
- `emoji.colormaps`

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
`label`, `shortcode` and `codepoint` are only required when an emoji uses those variables

The `name` of the colormap must start with `%`.

```toml
# Skin tone modifier
[[colormap]]
name = "%skin_tone.l1"
label = " - Light 1"
shortcode = "_l1"
codepoint = "U+1F3FB"
"$base.1" = "$skin_tone.l1"

# 3-color flag
[[colormap]]
name = "%flag_lt"
label = "Lithuania"
description = "The flag of Lithuania"
"$base.1" = "#FDBA0B"
"$base.2" = "#006A42"
"$base.3" = "#C22229"
```

### Emoji
- `name`, `description`, `category` - used for metadata
- `codepoint`: single codepoint split into parts
- `shortcodes`: list of shortcodes
- `tags`: used in targets to select which emojis to include in that target
- `src`: path to the svg file, relative to the manifest file
- `colormaps` create multiple emoji entries, one for each colormap. `%label`, `%shortcode`, `%codepoint`, `%description` will be replaced, and the svg will be recolored with the colormap's entries.

> If emojis have overlapping tags, they can't have overlapping names and labels

> If an emoji has multiple colormaps, `name`, `shortcodes` and `codepoint` must use variables

```toml
# Face
[[emoji]]
src = "./grinning-face.svg"
name = "Grinning face"
category = [ "expressions", "smileys" ]
description = "A smiling face with smiling eyes and open mouth."
tags = [ "unicode" ]
codepoint = [ "U+1F600" ]
shortcodes = [ "grinning" ]

# Non-unicode "extra" emoji with multiple color variations, with a private use area codepoint
[[emoji]]
src = "./eating-carrot.svg"
name = "Human eating a carrot%label"
category = [ "activities", "food" ]
description = "A human eating a carrot."
tags = [ "extra" ]
codepoint = [ "$pua", "U+1234", "%codepoint" ]
shortcodes = [ "human_eating_carrot%shortcode" ]
colormaps = [ "$skin_tone.all", "%skin_tone.l4" ] # notice the $ and % distinction

# Example 3 color flag using a template svg, colored with a colormap
[[emoji]]
src = "./base_flags/3_equal_horizontal_stripes.svg"
name = "%label"
category = [ "symbols", "flags" ]
description = "%description"
tags = [ "unicode" ]
codepoint = [ "U+1F1F1", "U+1F1F9" ]
shortcodes = [ "flag_lt", "flag_lithuania", "lithuania" ]
colormaps = [ "%flag_lt" ]
```
