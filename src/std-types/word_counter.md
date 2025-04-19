---
minutes: 20
---

# Exercise: Word Counter

Create a program that counts the frequency of words in a given text. The program should:

1. Take a string of text as input
2. Split the text into words (consider words to be separated by whitespace)
3. Count how many times each word appears (case-insensitive)
4. Print the words and their counts in alphabetical order

Use a `HashMap` to store the word counts.

## Example

```rust
fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    let counts = count_words(text);
    print_word_counts(&counts);
}
```

Expected output:
```
brown: 1
dog: 1
fox: 1
jumps: 1
lazy: 1
over: 1
quick: 1
the: 2
```

## Tasks

1. Implement the `count_words` function that takes a string slice and returns a `HashMap<String, u32>`
2. Make the word counting case-insensitive (e.g., "The" and "the" count as the same word)
3. Implement the `print_word_counts` function that prints the word counts in alphabetical order
4. Add tests for:
   - Empty input
   - Simple text with repeated words
   - Case-insensitive counting

## Extension Tasks (Optional)

1. Add support for reading text from a file
2. Add statistics like total words, unique words, and average word length
3. Find and display the most common words

[View solution](word_counter_solution.md)

```rust,editable
use std::collections::HashMap;

/// WordCounter counts the frequency of words in text.
struct WordCounter {
    word_counts: HashMap<String, usize>,
}

impl WordCounter {
    /// Create a new WordCounter.
    fn new() -> Self {
        todo!("Initialize the WordCounter")
    }

    /// Count words in the given text.
    fn count_words(&mut self, text: &str) {
        todo!("Implement word counting logic")
    }

    /// Get the count for a specific word.
    fn word_count(&self, word: &str) -> usize {
        todo!("Return the count for the given word")
    }

    /// Find the most frequent word(s) and their count.
    fn most_frequent(&self) -> Vec<(&str, usize)> {
        todo!("Find and return the most frequent word(s)")
    }
}

#[test]
fn test_word_counter() {
    let mut counter = WordCounter::new();
    counter.count_words("Hello world, hello Rust!");
    assert_eq!(counter.word_count("hello"), 2);
    assert_eq!(counter.word_count("rust"), 1);
    assert_eq!(counter.word_count("world"), 1);
} 