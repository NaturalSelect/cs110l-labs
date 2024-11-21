use std::env;
use std::fs::File;
use std::io::BufRead;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let fp = File::open(filename);
    if let Err(err) = fp {
        println!("Error: {}", err);
        process::exit(1);
    }
    let fp = fp.unwrap();
    let mut line_Cnt = 0;
    let mut word_Cnt = 0;
    let mut char_Cnt = 0;

    for line in std::io::BufReader::new(fp).lines() {
        if let Err(err) = line {
            println!("Error: {}", err);
            process::exit(1);
        }
        let line = line.unwrap();
        line_Cnt += 1;
        char_Cnt += line.len();
        word_Cnt += line.split_whitespace().count();
    }

    println!("{} {} {}", line_Cnt, word_Cnt, char_Cnt);
}
