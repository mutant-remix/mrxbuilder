# mrxbuilder

**Work in progress! Almost finished.**

**mrxbuilder** (Mutant Remix builder) is an emoji pack build tool. Takes in a TOML manifest and SVGs and outputs an emoji pack in various formats.

- Completely **emoji pack agnostic** with no hard-coded assumptions
- Lightning fast and multithreaded, no temporary files written to disk
- Cache friendly, only re-encodes what is necessary
- No runtime depdendencies
- Cross-platform, with prebuilt binaries for Linux, Windows and Mac OS


Check out [manifest specification](./manifest-specification.md) and [sample input](./sample-input) for more information on how to write a manifest.

If you are moving from **orxporter**, check out the [manifest porter](https://github.com/mutant-remix/manifest-porter) repository for a tool to semi-automatically convert your `orx` manifest to the new format.

## Features
- Recolors emojis using colormaps to avoid repeating SVGs with different colors
- Supports building to `svg`, `png`, `avif` and `webp` formats with various compression methods
- Outputs to a `directory` or directly to a `zip` file
- Output metadata is generated in the [Google fonts emoji metadata format](https://github.com/googlefonts/emoji-metadata) in JSON
- Really simple to run with only 3 arguments. Formats are pre-defined in the manifest, and selected for building using tags

## Usage
The builder is run from the command line. It takes exactly 3 arguments:
- path to the index manifest file
- output path
- tags for the targets to build (comma separated)

#### Prebuilt binaries
Download a prebuilt binary for your platform from the [releases page](https://github.com/mutant-remix/mrxbuilder/releases)

```bash
./builder ./input/index.toml ./output debug,release
```

#### Manual build
You will need to have Rust installed. The simplest way is to use [rustup](https://rustup.rs/).

```
cargo run --release -- ./manifest/index.toml ./out debug,release
```

> Note: Do not run it without the `--release` flag, as it will be **extremely** slow.

## Future plans
**Help wanted!**

- Support for more formats, such as `jpeg-xl`
- Support for writing EXIF metadata and svg metadata
- Support for fonts (likely never)

## License
Licensed under [AGPLv3](./LICENSE)
