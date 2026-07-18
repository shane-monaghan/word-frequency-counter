use std::fs;
use std::collections::HashMap;
use std::str::SplitWhitespace;

fn read_file(file_path: String) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read file");
}

fn main() {
    let contents : String = read_file("/workspaces/word-frequency-counter/word_freq_counter/src/odyssey_book1.txt".to_string());
    println!("{}", contents);

    let mut word_counts: HashMap<String, i32> = HashMap::new();
    let words: SplitWhitespace<'_> = contents.split_whitespace();

    for word in words {
        *word_counts.entry(word.to_string()).or_insert(0) += 1;
    }
    
    for k in word_counts.keys() {
        println!("Count of {} = {}", k.to_string(), word_counts[k].to_string());
    }
    
}