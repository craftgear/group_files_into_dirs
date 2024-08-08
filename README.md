# group_files_into_dir

a CLI tool to group files into directories based on their filenames.

## Installation

```bash
git clone https://github.com/craftgear/group_files_into_dirs
cargo build --release
```

## Usage

```bash
group_files_into_dir -k <keywords> <dir> 
```
- keywords: words to use for grouping files, comma separated.
- dir: directory to group files in.

## Example

```bash
# group files in current directory based on keywords "hello" and "world"
group_files_into_dir -k "hello,world" ./
```

## LICENSE
MIT License

