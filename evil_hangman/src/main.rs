use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let mut guessed: HashSet<char> = HashSet::new();
    let mut choices: HashMap<String, HashSet<String>> = HashMap::new();
    let mut words: HashSet<String> = get_words();
    let mut pattern: String = String::from("-");
    while pattern.contains("-") {
        println!("Guessed: {:?}", guessed);
        ask_for_char(&mut guessed);
        choices = update_word_choices(&words, &guessed);
        pattern = find_largest_group(&choices);
        println!("New Pattern: {}", pattern);
        words = choices.get(&pattern).unwrap().clone();
    }
    println!("Success! The word is {}", pattern);
}

fn ask_for_char(guessed: &mut HashSet<char>) {
    let mut input_string = String::new();
    println!("Next Guess: ");
    io::stdin().read_line(&mut input_string).unwrap();
    guessed.insert(input_string.chars().next().unwrap());
}

fn get_words() -> HashSet<String> {
    let dict_file: File =
        File::open("/Users/jacobmckenney/code/rust/evil_hangman/src/dictionary.txt").unwrap();
    let buf: BufReader<File> = io::BufReader::new(dict_file);
    let mut words: HashSet<String> = HashSet::new();
    for line in buf.lines() {
        words.insert(line.unwrap());
    }
    return words;
}

fn update_word_choices(
    words: &HashSet<String>,
    guessed: &HashSet<char>,
) -> HashMap<String, HashSet<String>> {
    let mut choices: HashMap<String, HashSet<String>> = HashMap::new();
    for word in words {
        let pattern = word_to_pattern(&word, guessed);
        let word_choices = choices.entry(pattern).or_insert(HashSet::new());
        word_choices.insert(word.clone());
    }
    return choices;
}

fn find_largest_group(choices: &HashMap<String, HashSet<String>>) -> String {
    let mut largest_pattern = String::from("");
    let mut largest_set_len: usize = 0;
    for (pattern, set) in choices {
        if set.len() > largest_set_len {
            largest_pattern = pattern.to_string();
            largest_set_len = set.len();
        }
    }
    return largest_pattern;
}

fn word_to_pattern(word: &str, guessed: &HashSet<char>) -> String {
    let mut result_word: String = String::with_capacity(word.len());
    for c in word.chars() {
        if guessed.contains(&c) {
            result_word.push(c)
        } else {
            result_word.push('-')
        }
    }
    return result_word;
}
