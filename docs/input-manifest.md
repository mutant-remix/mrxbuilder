# Manifest

## Overview
The builder's manifests are written in [toml](https://toml.io).

## References
- `#<abcdef>` - RGB hex color
- `U+<1234>` - Unicode codepoint
- `$<name>` - Variable name
- `%` - Colormaps:
    - `%<name>` - Colormap name
    - `%shortcode` exactly - Colormap shortcode (for example, `:circle-%shortcode:`)
    - `%codepoint` - Colormap codepoint (for example, skin tone modifiers)

> `<>` denotes a user-defined value, otherwise it is a literal

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
- `container`: `tgz`, `dir`
- `output_format`:
    - Vector images: `svg`
    - Raster images: `png`, `webp`, `avif` + width
    - Font: `ttf`, `woff2`, `otf`
- `tags`: Used when calling the builder to select which targets to build.

```toml
[[target]]
name = "full-shortcode-png-128"
tags = [ "debug" ]
include_tags = [ "unicode", "extra" ]
output_format = {
    container = "tgz",
    format = [ "png", 128 ]
}

[[target]]
name = "unicode-codepoint-svg"
tags = [ "production" ]
include_tags = [ "unicode" ]
output_format = {
    container = "dir",
    format = [ "svg" ]
}

[[target]]
name = "unicode-codepoint-ttf"
tags = [ "debug", "production" ]
include_tags = [ "unicode" ]
output_format = {
    container = "dir",
    format = [ "ttf" ]
}
```

### Define
Used to define variables for use in other parts of the manifest. The name of the variable will be matched with every instance of `$name` in the manifest (only in values, not keys)

This is also used for palette definitions.

```toml
# Often repeated codepoints
[[define]]
entries = [
    [ "$zwj", "U+200D" ], # zero-width joiner
    [ "$vs16", "U+FE0F" ], # variation selector 16
]

# Reusable palettes
[[define]]
entries = [
    [ "$skin_tone_1", "#123456" ],
    [ "$skin_tone_2", "#123456" ],
    [ "$skin_tone_3", "#123456" ],
]

# Merge multiple colormaps
[[define]]
entries = [
    [ "$skin_tone_all", "%skin_tone_1 %skin_tone_2 %skin_tone_3" ],
]
```

### Colormap
```toml
# Skin tone modifier
[[colormap]]
name = "skin_tone_1"
shortcode = "_l1" # light skin tone
codepoint = "U+1F3FB"
entries = [ # from-to
    [ '$base-1', '$skin_tone_1' ]
]

# 3-color flag
[[colormap]]
name = "flag_lt"
entries = [
    [ '$base-1', '#FDBA0B' ],
    [ '$base-2', '#006A42' ],
    [ '$base-3', '#C22229' ],
]
```

### Emoji
- `meta`: used for metadata
- `tags`: used in targets to select which emojis to include in that target

Adding colormaps to an emoji will create multiple emoji entries, one for each colormap. `%shortcode` and `%codepoint` will be replaced with the colormap's shortcode and codepoint respectively, and the svg will be recolored with the colormap's entries.

```toml
# Face
[[emoji]]
name = "grinning face"
description = "A smiling face with smiling eyes and open mouth."
src = "./grinning-face.svg"
tags = [ "unicode" ]
labels = {
    codepoint = [ "U+1F600" ],
    shortcodes = [ "grinning" ]
}

# Non-unicode emoji with multiple color variations
[[emoji]]
name = "eating carrot"
description = "A human eating a carrot"
src = "./eating-carrot.svg"
tags = [ "extra" ]
labels = {
    codepoint = [ "$pua", "U+1234", "%codepoint" ],
    shortcodes = [ "human_eating_carrot%shortcode" ]
}
# a variable referencing multiple colormaps + a single colormap
colormaps = "$skin_tone_all %skin_tone_4"

# Example 3 color flag using a template svg, colored with a colormap
[[emoji]]
name = "flag_lt"
description = "The flag of Lithuania"
src = "./base_flags/3_equal_horizontal_stripes.svg"
tags = [ "unicode" ]
labels = {
    codepoint = [ "U+1F1F1", "U+1F1F9" ],
    shortcodes = [ "flag_lt", "flag_lithuania", "lithuania" ]
}
colormaps = "%flag_lt"
```
