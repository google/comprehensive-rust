// Copyright 2023 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// ANCHOR: exercise
use std::collections::HashMap;

/// Count words in the given text and return a map of words to their counts.
/// Words are treated as case-insensitive.
fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();

    for word in text.split_whitespace() {
        // Remove any punctuation and convert to lowercase
        let word = word
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        if !word.is_empty() {
            *word_counts.entry(word).or_insert(0) += 1;
        }
    }

    word_counts
}

/// Print word counts in alphabetical order.
fn print_word_counts(counts: &HashMap<String, u32>) {
    let mut words: Vec<_> = counts.keys().collect();
    words.sort();

    for word in words {
        println!("{}: {}", word, counts[word]);
    }
}
// ANCHOR_END: exercise

// ANCHOR: word_counter
/// WordCounter counts the frequency of words in text.
struct WordCounter {
    word_counts: HashMap<String, usize>,
}

impl WordCounter {
    /// Create a new WordCounter.
    fn new() -> Self {
        WordCounter { word_counts: HashMap::new() }
    }

    /// Count words in the given text.
    fn count_words(&mut self, text: &str) {
        for word in text.split_whitespace() {
            // Remove any punctuation and convert to lowercase
            let word = word
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();

            if !word.is_empty() {
                *self.word_counts.entry(word).or_insert(0) += 1;
            }
        }
    }

    /// Get the count for a specific word.
    fn word_count(&self, word: &str) -> usize {
        self.word_counts.get(&word.to_lowercase()).copied().unwrap_or(0)
    }

    /// Find the most frequent word(s) and their count.
    fn most_frequent(&self) -> Vec<(&str, usize)> {
        if self.word_counts.is_empty() {
            return Vec::new();
        }

        let max_count = self.word_counts.values().max().unwrap();
        self.word_counts
            .iter()
            .filter(|(_, &count)| count == *max_count)
            .map(|(word, &count)| (word.as_str(), count))
            .collect()
    }

    /// Calculate statistics about the words.
    fn get_statistics(&self) -> WordStatistics {
        let total_words: usize = self.word_counts.values().sum();
        let unique_words = self.word_counts.len();

        let total_length: usize =
            self.word_counts.iter().map(|(word, count)| word.len() * count).sum();

        let avg_word_length = if total_words > 0 {
            total_length as f64 / total_words as f64
        } else {
            0.0
        };

        // Find the most common words (up to 5)
        let mut most_common: Vec<_> = self
            .word_counts
            .iter()
            .map(|(word, &count)| (word.to_string(), count))
            .collect();
        most_common.sort_by(|a, b| b.1.cmp(&a.1));
        most_common.truncate(5);

        WordStatistics { total_words, unique_words, avg_word_length, most_common }
    }
}

/// Statistics about words in a text.
struct WordStatistics {
    total_words: usize,
    unique_words: usize,
    avg_word_length: f64,
    most_common: Vec<(String, usize)>,
}
// ANCHOR_END: word_counter

// ANCHOR: tests
#[cfg(test)]
mod test {
    use super::*;

    // Tests for the basic word counting function
    #[test]
    fn test_count_words_function() {
        let text = "The quick brown fox jumps over the lazy dog";
        let counts = count_words(text);
        assert_eq!(counts.get("the").copied(), Some(2));
        assert_eq!(counts.get("quick").copied(), Some(1));
        assert_eq!(counts.get("missing").copied(), None);
    }

    // Tests for the WordCounter struct
    #[test]
    fn test_empty_counter() {
        let counter = WordCounter::new();
        assert_eq!(counter.word_count("any"), 0);
        assert!(counter.most_frequent().is_empty());
    }

    #[test]
    fn test_simple_text() {
        let mut counter = WordCounter::new();
        counter.count_words("Hello world, hello Rust!");
        assert_eq!(counter.word_count("hello"), 2);
        assert_eq!(counter.word_count("rust"), 1);
        assert_eq!(counter.word_count("world"), 1);
    }

    #[test]
    fn test_case_insensitive() {
        let mut counter = WordCounter::new();
        counter.count_words("Hello HELLO hello");
        assert_eq!(counter.word_count("hello"), 3);
        assert_eq!(counter.word_count("HELLO"), 3);
    }

    #[test]
    fn test_most_frequent() {
        let mut counter = WordCounter::new();
        counter.count_words("hello world hello rust hello");
        let most_frequent = counter.most_frequent();
        assert_eq!(most_frequent, vec![("hello", 3)]);
    }

    // Tests for the statistics functionality
    #[test]
    fn test_statistics() {
        let mut counter = WordCounter::new();
        counter.count_words("hello world hello rust hello programming");
        let stats = counter.get_statistics();
        assert_eq!(stats.total_words, 6);
        assert_eq!(stats.unique_words, 4);
        assert!(stats.avg_word_length > 0.0);
        assert_eq!(stats.most_common[0].0, "hello");
        assert_eq!(stats.most_common[0].1, 3);
    }
}
// ANCHOR_END: tests

// ANCHOR: main
fn main() {
    let text =
        "The quick brown fox jumps over the lazy dog. The fox is quick and brown.";
    let mut counter = WordCounter::new();
    counter.count_words(text);

    println!("Word Counts:");
    let mut words: Vec<_> = counter.word_counts.iter().collect();
    words.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0))); // Sort by frequency, then alphabetically

    for (word, count) in words {
        println!("{}: {}", word, count);
    }

    println!("\nStatistics:");
    let stats = counter.get_statistics();
    println!("Total words: {}", stats.total_words);
    println!("Unique words: {}", stats.unique_words);
    println!("Average word length: {:.2}", stats.avg_word_length);

    println!("\nMost common words:");
    for (word, count) in stats.most_common {
        println!("{}: {} occurrences", word, count);
    }
}
// ANCHOR_END: main
