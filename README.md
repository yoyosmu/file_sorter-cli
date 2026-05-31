# File Sorter-cli

A simple Rust program that automatically organizes files in a folder.

## Features

* Sorts images
* Sorts documents
* Sorts audio files
* Sorts archives

## ARGUMENTS

```bash
file_sorter-cli ~/Documents
file_sorter-cli --watch
file_sorter-cli --dry_run
```

## Usage

Run with:

```bash
cargo run
```

Or build release version:

```bash
cargo build --release
```

## Example

Before:

```text
Downloads/
├── image.png
├── music.mp3
├── video.mp4
├── file.pdf
├── archive.zip
```

After:

```text
Downloads/
├── PDFs/
├── IMGs/
├── AUDIOs/
├── VIDs/
├── ARCHIVEs/
```

## Built With

* Rust
* std::fs
* dirs crate
* clap features derive crate
