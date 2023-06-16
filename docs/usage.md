# Usage
## Overview
The builder loads a single manifest file (as described in [Input manifest](./input-manifest.md.md)) and builds the pack according to it to specified targets. Tags are used to select which targets to build.

The manifest file may load other manifest files using `[include]`

## Running the builder
The builder is run from the command line. It takes exactly 3 arguments:
- path to the index manifest file
- output path
- tags for the targets to build (comma separated)

### Prebuilt binaries
```bash
./builder_x86_64-unknown-linux-gnu ./manifest/index.toml ./out debug,production
# or
./builder_x86_64-pc-windows-gnu.exe ./manifest/index.toml ./out debug,production
```

### Manual build
You will need to have Rust installed. The simplest way is to use [rustup](https://rustup.rs/).

> Note: Do not run it without the `--release` flag, as it will be **extremely** slow.

```
cargo run --release -- ./manifest/index.toml ./out debug,production
```
