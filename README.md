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
This guide assumes general familiarity with the command line. mrxbuilder has no GUI, but there is not

mrxbuilder is run from the command line. It takes 3-4 arguments:
- path to the index manifest file
- output path (cache is also stored here)
- tags for the targets to build (comma separated)
- `--dry` flag to skip writing any files

### Prebuilt binaries
**Download** a prebuilt binary for your platform from the [releases page](https://github.com/mutant-remix/mrxbuilder/releases)

```bash
./mrxbuilder-v*-* ./sample-input/index.toml ./output debug,release [--dry]
```

### Manual build
#### Clone the repository
```bash
git clone https://github.com/mutant-remix/mrxbuilder
```

#### Dependencies
- Basic build tools
- Rust toolchain
- nasm (for building `rav1e`)

Windows 8+
> Note: You can use WSL instead
```bash
winget install -e --id=Rustlang.Rustup
winget install -e --id=NASM.NASM

# Command prompt
setx PATH "%PATH%;%USERPROFILE%\AppData\Local\bin\NASM\nasm.exe"
# Powershell
$env:Path += ";%USERPROFILE%\AppData\Local\bin\NASM\nasm.exe"

# Restart your terminal
rustup default stable-gnu # or 'stable-msvc' if you have Visual Studio
# Restart your terminal again
```

Debian-based Linux (Ubuntu, Pop!_OS, etc.)
```bash
apt install build-essential nasm curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # select 1
```

Arch-based Linux (Arch, Manjaro, etc.)
```bash
pacman -Sy base-devel rustup nasm
rustup default stable
```

Alpine Linux
```bash
apk add build-base rustup nasm
rustup-init # select 1
```

Mac OS
```bash
brew install nasm curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # select 1
```

#### Build and run
```bash
cargo run --release -- ./sample-input/index.toml ./out debug,release [--dry]

# or use mold for faster builds (linux only, optional)
mold -run cargo run --release -- ./sample-input/index.toml ./out debug,release [--dry]
```

> Note: Do not run it without the `--release` flag, as it will be **extremely** slow.

## Future plans
- Support for more formats, such as `jpeg-xl`
- Support for writing EXIF metadata and svg metadata. **help wanted**
- Support for fonts. **help wanted**

## License
Licensed under [AGPLv3](./LICENSE)
