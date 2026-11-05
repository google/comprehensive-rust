use std::collections::HashMap;
use std::fs;
use std::io;
use clap::Parser;
use colored::*;

/// Word Counter - A program to analyze word frequencies in text
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Text to analyze directly
    #[arg(short, long)]
    text: Option<String>,

    /// File to read text from
    #[arg(short, long)]
    file: Option<String>,

    /// Read from standard input
    #[arg(short, long)]
    interactive: bool,

    /// Case sensitive counting
    #[arg(short, long)]
    case_sensitive: bool,

    /// Show detailed statistics
    #[arg(short, long)]
    stats: bool,
}

#[derive(Default)]
struct Statistics {
    total_words: usize,
    unique_words: usize,
    avg_word_length: f64,
    most_common: Vec<(String, usize)>,
}

struct WordCounter {
    word_counts: HashMap<String, usize>,
    case_sensitive: bool,
}

impl WordCounter {
    fn new(case_sensitive: bool) -> Self {
        WordCounter {
            word_counts: HashMap::new(),
            case_sensitive,
        }
    }

    fn count_words(&mut self, text: &str) {
        for word in text.split_whitespace() {
            let word = if !self.case_sensitive {
                word.to_lowercase()
            } else {
                word.to_string()
            };
            *self.word_counts.entry(word).or_insert(0) += 1;
        }
    }

    fn get_statistics(&self) -> Statistics {
        let total_words: usize = self.word_counts.values().sum();
        let unique_words = self.word_counts.len();
        
        let total_length: usize = self.word_counts
            .iter()
            .map(|(word, count)| word.len() * count)
            .sum();
        
        let avg_word_length = if total_words > 0 {
            total_length as f64 / total_words as f64
        } else {
            0.0
        };

        let mut most_common: Vec<_> = self.word_counts
            .iter()
            .map(|(word, &count)| (word.clone(), count))
            .collect();
        most_common.sort_by(|a, b| b.1.cmp(&a.1));
        most_common.truncate(5);

        Statistics {
            total_words,
            unique_words,
            avg_word_length,
            most_common,
        }
    }

    fn print_results(&self, show_stats: bool) {
        println!("\n{}", "Word Counts:".green().bold());
        let mut words: Vec<_> = self.word_counts.iter().collect();
        words.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

        for (word, count) in words {
            println!("{}: {}", word.cyan(), count.to_string().yellow());
        }

        if show_stats {
            let stats = self.get_statistics();
            println!("\n{}", "Statistics:".green().bold());
            println!("Total words: {}", stats.total_words.to_string().yellow());
            println!("Unique words: {}", stats.unique_words.to_string().yellow());
            println!("Average word length: {:.2}", stats.avg_word_length.to_string().yellow());
            
            println!("\n{}", "Most common words:".green().bold());
            for (word, count) in stats.most_common {
                println!("{}: {} occurrences", word.cyan(), count.to_string().yellow());
            }
        }
    }
}

fn interactive_mode(counter: &mut WordCounter, _show_stats: bool) {
    println!("{}", "\nInteractive Mode - Enter text (press Ctrl+D or Ctrl+Z to finish):".green().bold());
    let stdin = io::stdin();
    let mut buffer = String::new();
    
    while stdin.read_line(&mut buffer).unwrap_or(0) > 0 {
        counter.count_words(&buffer);
        buffer.clear();
    }
}

fn main() {
    let args = Args::parse();
    let mut counter = WordCounter::new(!args.case_sensitive);

    if args.interactive {
        interactive_mode(&mut counter, args.stats);
    } else if let Some(text) = args.text {
        counter.count_words(&text);
    } else if let Some(file) = args.file {
        match fs::read_to_string(file) {
            Ok(content) => counter.count_words(&content),
            Err(e) => {
                eprintln!("{}: {}", "Error reading file".red().bold(), e);
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{}", "Please provide --text, --file, or use --interactive mode".red().bold());
        std::process::exit(1);
    }

    counter.print_results(args.stats);
} 