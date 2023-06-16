# mrxbuilder

**Work in progress!**

mrxbuilder (Mutant Remix builder) is an emoji pack build tool. Takes in a manifest file and SVGs and outputs a pack in various formats.

- Completely **emoji pack agnostic** with no hard-coded assumptions
- Fast and multithreaded, no temporary files are written to disk
- No runtime depdendencies
- Really simple to run with only 3 arguments

Check out the [docs](./docs/README.md) for more information.

## Features
- Recolors emojis using colormaps to avoid repeating SVGs with different colors
- Supports building to `svg`, `png`, `avif` and `webp` formats with various compression methods
- Outputs artifacts in `tar.gz`, `zip` or a directory
- Output metadata is generated in JSON

## Metadata
- Input metadata is written in a simple, human-readable format using [toml](https://toml.io)
> Read more about the metadata formats in the [docs](./docs/README.md)
- Build targets can be pre-defined in the manifest, and selected for building using tags

## License
Licensed under [AGPLv3](./LICENSE)
