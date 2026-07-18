use std::fs;

fn read_file(file_path: String) -> String {
    return fs::read_to_string(file_path).expect("Should have been able to read file");
}

fn main() {
    let contents : String = read_file("/workspaces/word-frequency-counter/word_freq_counter/src/odyssey_book1.txt".to_string());
    println!("{}", contents);
}