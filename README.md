# mrxbuilder

**Work in progress! Almost finished.**

**mrxbuilder** (Mutant Remix builder) is an emoji pack build tool. Takes in a TOML manifest and SVGs and outputs an emoji pack in various formats.

- Completely **emoji pack agnostic** with no hard-coded assumptions
- Lightning fast and multithreaded, no temporary files written to disk
- Cache friendly, only re-encodes what is necessary
- No runtime depdendencies
- Cross-platform, with prebuilt binaries for Linux, Windows and Mac OS

If you are moving from **orxporter**, check out the [manifest porter](https://github.com/mutant-remix/manifest-porter) repository for a tool to semi-automatically convert your `orx` manifest to the new format.

## Features
- Recolors emojis using colormaps to avoid repeating SVGs with different colors
- Supports building to `svg`, `png`, `avif` and `webp` formats with various compression methods
- Outputs to a `directory` or directly to a `zip`/`tar` file with various compression methods
- Really simple to run with only 3 arguments. Formats are pre-defined in the manifest, and selected for building using tags

## Manifest
Check out the [documentation](./docs) and [sample input](./sample-input) for input manifest and output metadata specifications and examples.

## Usage
The builder is run from the command line. It takes 3-4 arguments:
- path to the index manifest file
- output path (cache is also stored here)
- tags for the targets to build (comma separated)
- `--dry` flag to skip writing any files

#### Prebuilt binaries
Download a prebuilt binary for your platform from the [releases page](https://github.com/mutant-remix/mrxbuilder/releases)

```bash
./builder ./input/index.toml ./output debug,release [--dry]
```

#### Manual build
You will need to have Rust installed. The simplest way is to use [rustup](https://rustup.rs/).

```
cargo run --release -- ./manifest/index.toml ./out debug,release [--dry]
```

> Note: Do not run it without the `--release` flag, as it will be **extremely** slow.

## Future plans
- Support for more formats, such as `jpeg-xl`
- Support for writing EXIF metadata and svg metadata. **help wanted**
- Support for fonts. **help wanted**

## License
Licensed under [AGPLv3](./LICENSE)
