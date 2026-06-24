---
minutes: 20
---

# Exercise: Word Counter

Create a program that counts the frequency of words in a given text. The program
should:

1. Take a string of text as input
2. Split the text into words (consider words to be separated by whitespace)
3. Count how many times each word appears (case-insensitive)
4. Print the words and their counts in alphabetical order

Use a `HashMap` to store the word counts.

## Example

```rust
{{#include word_counter.rs:main}}
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

1. Implement the `count_words` function that takes a string slice and returns a
   `HashMap<String, u32>`
2. Make the word counting case-insensitive (e.g., "The" and "the" count as the
   same word)
3. Implement the `print_word_counts` function that prints the word counts in
   alphabetical order
4. Add tests for:
   - Empty input
   - Simple text with repeated words
   - Case-insensitive counting

## Extension Tasks (Optional)

1. Add support for reading text from a file
2. Add statistics like total words, unique words, and average word length
3. Find and display the most common words

```rust,editable
{{#include word_counter.rs:exercise}}
```
