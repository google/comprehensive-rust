use std::collections::HashMap;
use std::error::Error;

fn count_words(text: &str) -> Result<HashMap<String, u32>, Box<dyn Error>> {
    if text.trim().is_empty() {
        return Err("Empty input".into());
    }

    let mut word_counts = HashMap::new();
    
    for word in text.split_whitespace() {
        *word_counts.entry(word.to_lowercase()).or_insert(0) += 1;
    }
    
    Ok(word_counts)
}

fn print_word_counts(counts: &HashMap<String, u32>) {
    let mut words: Vec<_> = counts.keys().collect();
    words.sort();
    
    for word in words {
        println!("{}: {}", word, counts[word]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let text = "The quick brown fox jumps over the lazy dog";
    let counts = count_words(text)?;
    print_word_counts(&counts);
    
    // Test empty input
    match count_words("") {
        Ok(_) => println!("Unexpected success with empty input"),
        Err(e) => println!("Expected error: {}", e),
    }
    
    Ok(())
}

struct WordCounter {
    word_counts: HashMap<String, usize>,
}

impl WordCounter {
    fn new() -> Self {
        WordCounter {
            word_counts: HashMap::new(),
        }
    }

    fn count_words(&mut self, text: &str) {
        // Convert to lowercase and split into words
        for word in text.to_lowercase()
            .split(|c: char| !c.is_alphabetic())
            .filter(|s| !s.is_empty())
        {
            *self.word_counts.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    fn word_count(&self, word: &str) -> usize {
        self.word_counts.get(&word.to_lowercase()).copied().unwrap_or(0)
    }

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
}

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