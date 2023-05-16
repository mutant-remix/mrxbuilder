# Usage
## Overview
The builder loads a single manifest file (as described in [Input manifest](./input-manifest.md.md)) and builds the pack according to it to specified targets. Tags are used to select which targets to build.

The manifest file may loader other manifest files using `[include]`

## Running the builder
The builder is run from the command line. It takes exactly 3 arguments:
- path to the index manifest file
- output path
- tags for the targets to build (comma separated)

```bash
./builder_x86_64-unknown-linux-gnu ./manifest/index.toml ./out debug,production
# or
./builder_x86_64-pc-windows-gnu.exe ./manifest/index.toml ./out debug,production
# or
cargo run --release -- ./manifest/index.toml ./out debug,production
```
