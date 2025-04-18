use std::collections::HashMap;

/// Count the frequency of words in a text.
/// Returns a HashMap with words as keys and their counts as values.
fn count_words(text: &str) -> HashMap<String, u32> {
    let mut word_counts = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *word_counts.entry(word).or_insert(0) += 1;
    }
    
    word_counts
}

/// Print word counts in alphabetical order
fn print_word_counts(counts: &HashMap<String, u32>) {
    let mut words: Vec<_> = counts.keys().collect();
    words.sort();
    
    for word in words {
        println!("{}: {}", word, counts[word]);
    }
}

#[test]
fn test_empty_string() {
    let counts = count_words("");
    assert!(counts.is_empty());
}

#[test]
fn test_simple_text() {
    let counts = count_words("the quick brown fox jumps over the lazy dog");
    assert_eq!(counts["the"], 2);
    assert_eq!(counts["fox"], 1);
    assert_eq!(counts.len(), 8);
}

#[test]
fn test_case_insensitive() {
    let counts = count_words("The THE the");
    assert_eq!(counts["the"], 3);
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    let counts = count_words(text);
    print_word_counts(&counts);
} 