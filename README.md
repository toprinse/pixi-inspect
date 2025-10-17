# pixi-inspect

This project provides a command-line tool to inspect conda packages and extract metadata from their `index.json` file.  
It is designed to work well with [Pixi](https://pixi.sh/latest/) and CI/CD workflows.

## Description

`pixi-inspect` analyzes a conda package (either from disk or stdin) and extracts metadata from its `index.json`.  
It uses the [rattler_package_streaming](https://github.com/mamba-org/rattler) crate for robust extraction and parsing.

## Installation

### Prerequisites
- Rust 1.70+ and Cargo
- [Pixi](https://pixi.sh/latest/) (optional, for conda workflows)

### Compilation
```bash
git clone https://gitlab.in2p3.fr/thibaut.oprinsen/pixi-inspect
cd pixi-inspect
cargo build --release
```
The binary will be available at `target/release/pixi-inspect`.

To use globally:
```bash
cp target/release/pixi-inspect ~/.local/bin/
```
Make sure `~/.local/bin/` is in your `PATH`.

## Usage

### Inspect a single package
```bash
pixi-inspect get-info /path/to/package.conda
```

### Inspect a package from stdin
```bash
cat package.conda | pixi-inspect get-info -
```

### Inspect all packages in a workspace (excluding `.pixi`)
```bash
find /path/to/workspace -path '/path/to/workspace/.pixi' -prune -o -name '*.conda' -exec pixi-inspect get-info {} \;
```

### Inspect all packages in the current directory
```bash
find . -path './.pixi' -prune -o -name '*.conda' -exec pixi-inspect get-info {} \;
```

### Help
```bash
pixi-inspect --help
```

## Features

- ✅ Extracts metadata from `.conda` and `.tar.bz2` packages
- ✅ Reads from file or stdin
- ✅ Displays pretty-printed JSON
- ✅ Robust error handling
- ✅ Automatic cleanup of temporary files

## Metadata Structure

A typical `index.json` looks like:
```json
{
  "name": "package-name",
  "version": "1.0.0",
  "build": "py39h123456_0",
  "build_number": 0,
  "depends": ["python >=3.9", "numpy"],
  "platform": "linux-64",
  "license": "MIT",
  "timestamp": 1634567890,
  "size": 1234567
}
```

## Code Architecture

```
src/
└── main.rs          # CLI entry point, extraction and parsing logic
```
> Extraction and parsing are now handled directly in `main.rs` using rattler_package_streaming.  

## Use Cases

### CI Integration
```bash
PACKAGE_VERSION=$(pixi-inspect get-info /path/to/package.conda | jq -r '.version')
echo "Package version: $PACKAGE_VERSION"
```

### Dependency Analysis
```bash
pixi-inspect get-info /path/to/package.conda | jq -r '.depends[]'
```