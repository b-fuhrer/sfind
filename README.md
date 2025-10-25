# sfind ⚡

A simple, fast, and efficient command-line tool for finding files based on filename or content substrings.

-----

## Features

* Recursively searches directories.
* Filters by filename substring (`-f`).
* Filters by file content substring (`-c`), using optimized byte search and skipping binary files.
* Parallel search execution for high performance.
* Optional reporting of I/O errors during content search (`-e`).

-----

## Installation

```bash
cargo install sfind
```

*(Ensure `$HOME/.cargo/bin` is in your PATH)*

-----

## Usage

```bash
sfind [OPTIONS]
```

**Required:** You must provide at least one filter: `-f <filename_substring>` or `-c <content_substring>`.

**Options:**

* `-d, --dir <DIRECTORY>`: The directory to start the search from (defaults to current directory `.`).
* `-f, --file <SUBSTRING>`: Substring that must be present in the filename.
* `-c, --content <SUBSTRING>`: Substring that must be present in the file's content.
* `-e, --errors`: Show I/O errors encountered while reading file content (requires `-c`).

**Examples:**

* Find all `.toml` files in the current directory and subdirectories:
  ```bash
  sfind -f ".toml"
  ```
* Find all files under `src/` containing the word `ErrorPolicy`:
  ```bash
  sfind -d src/ -c "ErrorPolicy"
  ```
* Find all `.rs` files containing `TODO`, showing any I/O errors:
  ```bash
  sfind -f ".rs" -c "TODO" -e
  ```
