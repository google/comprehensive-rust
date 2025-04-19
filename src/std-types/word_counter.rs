// ANCHOR: word_counter
use std::collections::HashMap;
use clap::Parser;

/// A word counting program
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to count words in
    #[arg(short, long)]
    text: Option<String>,

    /// File to read text from
    #[arg(short, long)]
    file: Option<String>,

    /// Ignore case when counting words
    #[arg(short, long, default_value_t = true)]
    ignore_case: bool,
}

/// WordCounter counts the frequency of words in text.
struct WordCounter {
    word_counts: HashMap<String, usize>,
    ignore_case: bool,
}

impl WordCounter {
    /// Create a new WordCounter.
    fn new(ignore_case: bool) -> Self {
        WordCounter {
            word_counts: HashMap::new(),
            ignore_case,
        }
    }

    /// Count words in the given text.
    fn count_words(&mut self, text: &str) {
        for word in text.split_whitespace() {
            let word = if self.ignore_case {
                word.to_lowercase()
            } else {
                word.to_string()
            };
            *self.word_counts.entry(word).or_insert(0) += 1;
        }
    }

    /// Get the count for a specific word.
    fn word_count(&self, word: &str) -> usize {
        let word = if self.ignore_case {
            word.to_lowercase()
        } else {
            word.to_string()
        };
        self.word_counts.get(&word).copied().unwrap_or(0)
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

    /// Print word counts in alphabetical order
    fn print_counts(&self) {
        let mut words: Vec<_> = self.word_counts.keys().collect();
        words.sort();
        for word in words {
            println!("{}: {}", word, self.word_counts[word]);
        }
    }
}
// ANCHOR_END: word_counter

// ANCHOR: tests
#[test]
fn test_empty_counter() {
    let counter = WordCounter::new(true);
    assert_eq!(counter.word_count("any"), 0);
    assert!(counter.most_frequent().is_empty());
}

#[test]
fn test_simple_text() {
    let mut counter = WordCounter::new(true);
    counter.count_words("Hello world, hello Rust!");
    assert_eq!(counter.word_count("hello"), 2);
    assert_eq!(counter.word_count("rust"), 1);
    assert_eq!(counter.word_count("world"), 1);
}

#[test]
fn test_case_insensitive() {
    let mut counter = WordCounter::new(true);
    counter.count_words("Hello HELLO hello");
    assert_eq!(counter.word_count("hello"), 3);
    assert_eq!(counter.word_count("HELLO"), 3);
}

#[test]
fn test_most_frequent() {
    let mut counter = WordCounter::new(true);
    counter.count_words("hello world hello rust hello");
    let most_frequent = counter.most_frequent();
    assert_eq!(most_frequent, vec![("hello", 3)]);
}
// ANCHOR_END: tests

fn main() {
    let args = Args::parse();
    
    let mut counter = WordCounter::new(args.ignore_case);

    if let Some(text) = args.text {
        counter.count_words(&text);
    } else if let Some(file) = args.file {
        match std::fs::read_to_string(file) {
            Ok(content) => counter.count_words(&content),
            Err(e) => {
                eprintln!("Error reading file: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("Please provide either --text or --file");
        std::process::exit(1);
    }

    println!("Word counts:");
    counter.print_counts();
} 