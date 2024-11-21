// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn get_masked_secret_word(chars: &Vec<char>, show_table: &Vec<bool>) -> String {
    let mut r = String::new();
    r.reserve(chars.len());
    for i in 0..chars.len() {
        if show_table[i] {
            r.push(chars[i]);
        } else {
            r.push('_');
        }
    }
    return r;
}

fn handle_guess(guess: char, secret_word_chars: &Vec<char>, show_table: &mut Vec<bool>) -> bool {
    for i in 0..secret_word_chars.len() {
        if secret_word_chars[i] == guess && !show_table[i] {
            show_table[i] = true;
            return true;
        }
    }
    return false;
}

fn checkWin(show_table: &Vec<bool>) -> bool {
    for i in 0..show_table.len() {
        if !show_table[i] {
            return false;
        }
    }
    return true;
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    // Uncomment for debugging:
    println!("random word: {}", secret_word);

    // Your code here! :)
    let mut show_table: Vec<bool> = Vec::new();
    show_table.resize(secret_word_chars.len(), false);

    let guessed_letters = String::new();

    let mut cnt = 5;

    println!("Welcome to CS110L Hangman!");
    loop {
        println!(
            "The word so far is {}\n",
            get_masked_secret_word(&secret_word_chars, &show_table)
        );
        println!(
            "You have guessed the following letters: {}",
            guessed_letters
        );
        println!("You have {} guesses left", cnt);
        print!("Please guess a letter: ");
        io::stdout().flush().expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");
        let first_char = guess.chars().next().unwrap();
        if !handle_guess(first_char, &secret_word_chars, &mut show_table) {
            cnt -= 1;
            println!("Sorry, that letter is not in the word");
        }
        if checkWin(&show_table) {
            println!("Congratulations, you guessed the secret word");
            return;
        }
        if cnt == 0 {
            println!("Sorry, you ran out of guesses!");
            return;
        }
    }
}
