# Word Counter

A Rust program that analyzes word frequencies in text, providing detailed statistics and various input methods.

## Features

- Multiple input methods:
  - Direct text input
  - File input
  - Interactive mode
- Case-sensitive or case-insensitive word counting
- Detailed statistics:
  - Total word count
  - Unique word count
  - Average word length
  - Most common words
- Colored output for better readability
- Error handling for file operations

## Usage

```bash
# Count words in text
cargo run -- --text "Your text here"

# Count words from a file
cargo run -- --file "path/to/your/file.txt"

# Use interactive mode
cargo run -- --interactive

# Show detailed statistics
cargo run -- --text "Your text here" --stats

# Enable case-sensitive counting
cargo run -- --text "Your text here" --case-sensitive
```

### Command Line Options

- `--text, -t`: Directly input text to analyze
- `--file, -f`: Specify a file to read text from
- `--interactive, -i`: Enter interactive mode for continuous input
- `--case-sensitive, -c`: Enable case-sensitive word counting
- `--stats, -s`: Show detailed statistics

## Installation

1. Make sure you have Rust installed
2. Clone the repository
3. Run `cargo build --release`
4. The binary will be available in `target/release/word_counter`

## Example

```bash
cargo run -- --text "The quick brown fox jumps over the lazy dog" --stats
```

This will output:
- Word frequency counts
- Total word count
- Number of unique words
- Average word length
- Most common words

## Contributing

This is part of the Comprehensive Rust course. Feel free to submit issues and enhancement requests.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 