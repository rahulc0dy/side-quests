# PREG - Pure Rust Example of Grep

A simple grep-like utility implemented in Rust.

## Features

- Search for patterns in files or from stdin
- Case-insensitive search option
- Display line numbers
- Count matching lines
- Show only matching portions of lines
- Invert matches (show non-matching lines)
- Colorized output

## Usage

```
preg --pattern <PATTERN> --filename <FILE> [OPTIONS]
```

### Options

- `-p, --pattern <PATTERN>` - The pattern to search for
- `-f, --filename <FILE>` - The file to search (use "-" for stdin)
- `-i, --ignore-case` - Case insensitive search
- `-n, --line-numbers` - Show line numbers
- `-c, --count` - Only show count of matching lines
- `-o, --only-matching` - Show only matching part of the line
- `-v, --invert-match` - Show non-matching lines
- `--color <always|never|auto>` - Control colorized output (default: auto)

### Examples

```bash
# Search for "example" in file.txt
preg --pattern example --filename file.txt

# Case-insensitive search with line numbers
preg -p example -f file.txt -i -n

# Count matches
preg -p example -f file.txt -c

# Search from stdin
cat file.txt | preg -p example -f -

# Show only matching parts
preg -p example -f file.txt -o

# Show non-matching lines
preg -p example -f file.txt -v
```

## License

MIT
