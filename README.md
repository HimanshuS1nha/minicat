# minicat

A mini clone of the `cat` command written in Rust.

## Usage

- Clone the repo

```bash
git clone https://github.com/HimanshuS1nha/minicat.git
```

- Build a release version using cargo

```bash
cd minicat
cargo build --release
```

- Run the binary

```bash
./target/release/minicat filename
./target/release/minicat -n filename
```

## Features

- Concatenate one or more UTF-8 text files to standard output in the order provided.
- Preserve original line endings without adding a trailing newline.
- `-n`, `--number`: Number all lines.
- `-b`, `--number-nonblank`: Number only nonempty lines. Overrides `-n` when both are supplied.
- `-E`, `--show-ends`: Display `$` before each newline.
- `-T`, `--show-tabs`: Display tab characters as `^I`.
- `-A`, `--show-all`: Combine `--show-ends` and `--show-tabs`.
- `-h`, `--help`: Show usage and available options.
- Report errors to standard error and exit with a nonzero status.

## Demo

Live Demo - Coming soon
