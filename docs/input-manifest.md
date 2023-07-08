# Manifest
This is the specification for mrxbuilder's input manifest files, which also acts as the documentation. It is example-based and hopefully self-explanatory in most cases.

## Examples
You can find examples utilising every feature in [sample-input](./sample-input) and below.

## Overview
The builder's manifests are written in [toml](https://toml.io).

There are only a 5 entry types:
- [Include](#include) - Loads other manifest files
- [Target](#target) - Defines various outputs to build
- [Define](#define) - Defines variables for use in other parts of the manifest
- [Colormap](#colormap) - Defines a colormap to be used in emojis to avoid most repetition
- [Emoji](#emoji) - Defines an emoji

## Notes
Paths are relative to each manifest file.

## Literals and variables
- `#<abcdef>` - RGB hex color
- `U+<1234>` - Unicode codepoint
- `$<name>` - Variable name
- `%` - Colormaps:
    - `%<name>` - Colormap name
    - `%label` - Colormap label (for example, skin tone modifiers)
    - `%shortcode` - Colormap shortcode (for example, `circle%shortcode`)
    - `%codepoint` - Colormap codepoint (for example, skin tone modifiers)

> `<>` denotes a user-defined value, otherwise it is a literal (don't include the `<>`)

## Entry types
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
- `tags`: Used when calling mrxbuilder to select which targets to build.
- `output`:
    - `format`:
        - No images: `none` - used for metadata-only builds
        - Vector images: `svg` - skips rasterization
        - Raster images:
            Format name | Compression levels | Size | Compatibility | Speed | Notes
            --- | --- | --- | --- | --- | ---
            `png-image` | n/a | Huge | Best | Fast | **recommended for development**
            `png-oxipng-zopfli` | 0.0-14.0 | Tiny | Best | Slow | **recommended for very low resolutions**
            `png-oxipng-libdeflater` | 0.0-12.0 | Small | Best | Medium | **recommended**
            `webp` | n/a | Small | Modern browsers | Fast |
            `avif-lossy` | 100.0-0.0 | Small | Bad | **Very** slow | At high quality levels, it is not perceptibly lossy
    - `size` (number) - only for raster images
    - `compression` (number) - for applicable formats
- `structure`
    - `container`
        Name | Container | Extension | Compression | Notes
        --- | --- | --- | --- | ---
        `directory` | Directory | n/a | n/a | **recommended for development**
        `zip` | ZIP | `.zip` | none |
        `zip-deflate` | ZIP | `.zip` | Deflate | **recommended**
        `zip-bz2` | ZIP | `.bz2.zip` | Bzip2 | **low compatibility**
        `zip-zst` | ZIP | `.zst.zip` | Zstandard | **low compatibility**
        `tar` | TAR | `.tar` | none |
        `tar-gz` | TAR | `.tar.gz` | Gzip |
        `tar-bz2` | TAR | `.tar.bz2` | Bzip2 |
        `tar-xz` | TAR | `.tar.xz` | XZ |
        `tar-zst` | TAR | `.tar.zst` | Zstandard | Smallest size
    - `flat`
        - `true` - all emojis in the same directory
        - `false` - emojis in subdirectories by category
    - `filenames`
        - `shortcode` - the first shortcode as the filename
        - `codepoint` - the full codepoint **in base 10** (joined with `-`) as the filename (typically with `structure.flat = true`)
- `include_files` - array of paths to files to include in the output

```toml
[[target]]
name = "full-shortcode-png-128"
tags = [ "release" ]
include_tags = [ "unicode", "extra" ]
output = { format = "png-oxipng-libdeflater", size = 128, compression = 12.0 }
structure = { container = "tar.gz", subdirectories = true, filenames = "shortcode" }
include_files = [ "./LICENSE" ]

[[target]]
name = "unicode-codepoint-svg"
tags = [ "debug", "release" ]
include_tags = [ "unicode" ]
output = { format = "svg" }
structure = { container = "directory", subdirectories = false, filenames = "codepoint" }
include_files = [ "./LICENSE" ]

[[target]]
name = "full-metadata"
tags = [ "metadata" ]
include_tags = [ "unicode", "extra" ]
output = { format = "none" }
structure = { container = "directory", subdirectories = false, filenames = "shortcode" }
include_files = [ "./LICENSE" ]
```

### Define
Used to define variables for use in other parts of the manifest. The name of the variable will be matched with every instance of `$name` in the manifest (only in values, not keys)

This is also used for palette definitions.

The `name` of the variable must start with `$`.

`$` variables will only get resolved in:
- `colormap` entry keys and values
- `colormap.codepoint` (**not** expanded)
- `emoji.codepoint` (expanded)
- `emoji.colormaps` (expanded)

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
`label`, `shortcode` and `codepoint` are only required when an emoji uses those variables, otherwise the build will fail.

The `name` of the colormap must start with `%`

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
- `name` - used for metadata
- `description` - used for metadata
- `category` - used for metadata and structure
> The first `category` will be used as the group in the metadata
- `codepoint`: single codepoint split into parts, each starting with `U+`
> If `structure.filenames` is `codepoint`, this will be used as the filename, with `U+` stripped and joined with `_`
- `shortcodes`: list of shortcodes.
> The first one will be used as the filename if `structure.filenames` is `shortcode`
- `tags`: used to select which targets this emoji will be built for
- `src`: path to the svg file, **relative to the manifest file**
- `colormaps` create multiple emoji entries, one for each colormap.
> `%label`, `%shortcode`, `%codepoint`, `%description` will be replaced, and the svg will be recolored with the colormap's entries.

> If emojis have overlapping tags, they can't have overlapping names and labels

> If an emoji has multiple colormaps, `name`, `shortcodes` and `codepoint` must use `%` variables to avoid name collisions

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
