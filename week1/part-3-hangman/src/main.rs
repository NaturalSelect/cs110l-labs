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
use std::char;
use std::fs;
use std::io;
use std::io::Write;
use std::mem::swap;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let mut secret_word_chars: Vec<char> = secret_word.chars().collect();
    let mut output: Vec<char> = vec!['-'; secret_word_chars.len()];
    print!("Please guess a letter: ");
    // Make sure the prompt from the previous line gets displayed:
    io::stdout().flush().expect("Error flushing stdout.");
    let mut state = 0;
    let mut chance = NUM_INCORRECT_GUESSES;
    let mut rest = secret_word_chars.len();
    loop  {
        print!("Please guess a letter: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Error reading line.");
        if guess.len() > 3 {
            state = 3;
            break;
        }
        let guess_char = guess.chars().next().unwrap();
        match secret_word_chars.iter().position(|x| *x == guess_char) {
            Some(i) => {
            output[i] =  secret_word_chars[i];
            secret_word_chars[i] = '-';
            rest -=1;},
            None => {chance -= 1;}
        }
        if chance == 0 { 
            state = 2;
            break;
        } 
        if rest == 0 {
            state = 1;
            break;
        }
        output.iter().for_each(|c| {print!("{}", c)});
        println!("");
    }
    if state == 1 {
        println!("win!");
    } else {
        println!("lose!{}, word:{}", state, secret_word);
    };
}
