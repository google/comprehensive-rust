# Solution: Word Counter

Here's a solution for the Word Counter exercise using a `HashMap` to track word
frequencies:

```rust
{{#include word_counter.rs:exercise}}
```

This solution:

1. Implements `count_words` which:
   - Creates an empty `HashMap` to store word counts
   - Iterates through words in the input text (split by whitespace)
   - Converts each word to lowercase for case-insensitive counting
   - Uses the `entry` API to increment the count for each word

2. Implements `print_word_counts` which:
   - Collects and sorts the keys (words) alphabetically
   - Prints each word with its count

## Advanced Implementation

For more advanced needs, a `WordCounter` struct provides additional
functionality:

```rust
{{#include word_counter.rs:word_counter}}
```

The struct-based approach offers more methods like `most_frequent()` which finds
the most common word(s) in the text and `get_statistics()` which calculates
various metrics about the words.

## Statistics Example

The `WordStatistics` struct and the `get_statistics()` method show how to
calculate:

- Total word count
- Unique word count
- Average word length
- Most frequently used words

## Key Learning Points

1. **Using HashMaps**: The solution demonstrates how to use a HashMap to
   associate words with their counts.

2. **Entry API**: The code uses `entry().or_insert(0)` to efficiently handle the
   case where a word is seen for the first time.

3. **String manipulation**: Words are converted to lowercase for
   case-insensitive comparison and punctuation is filtered out.

4. **Sorting and iteration**: The solution shows how to collect keys from a
   HashMap, sort them, and iterate through them.

5. **Testing**: The comprehensive tests verify word counting,
   case-insensitivity, punctuation handling, and statistics calculations.
