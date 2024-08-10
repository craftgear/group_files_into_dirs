# group_files_into_dir

a CLI tool to group files into directories based on their filenames.

## Installation

```bash
git clone https://github.com/craftgear/group_files_into_dirs
cargo build --release
```

## Usage

### Interactive mode

extract keywords from filenames.
delimiters are `,_- ` and braces.

- dir: directory to group files in.

```bash
group_files_into_dir <dir>
```
[interactive mode](./images/interactive_mode.jpeg)



### specify keywords by yourself

- keywords: words to use for grouping files, comma separated.
- dir: directory to group files in.

```bash
group_files_into_dir -k <keywords> <dir> 
```

## Example

```bash
# invoke interactive mode (default)
group_files_into_dir ./

# group files in current directory based on keywords "hello" and "world"
group_files_into_dir -k "hello,world" ./
```

## LICENSE
MIT License

