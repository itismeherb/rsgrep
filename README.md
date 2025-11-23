# rsgrep
**rsgrep** is a fast, parallel, and user-friendly text search tool written in Rust.  
It provides grep-like functionality with modern enhancements for speed, usability, and readability.

Licensed under the [UNILICENSE](https://unlicense.org/)

## Changelog
For a complete release history, please refer to the [CHANGELOG](CHANGELOG.md).

## Requirements
- **Rust**: `rsgrep` requires Rust **1.85.0 (stable)** or newer.  
- You can install Rust [here](https://rust-lang.org/).

## Building from source
To compile **rsgrep** from source:

```bash
git clone https://github.com/itismeherb/rsgrep.git
cd rsgrep
cargo build --release
```

The compiled binary will be available at:

```text
target/release/rsgrep
```

## Usage
After building or installing, you can run **rsgrep** like this:

```bash
rsgrep [OPTIONS] <pattern> <path>
```

Example:

```bash
rsgrep -i "TODO" ~/.config/nvim
```

- `-i` or `--ignore-case` enables case-insensitive search
- Outputs matching lines with **highlighted matches** (requires terminal with color support)
