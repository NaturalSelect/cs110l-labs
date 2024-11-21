use grid::Grid; // For lcs()
use std::env;
use std::fs::{read, File}; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let fp = File::open(filename);
    if let Err(err) = fp {
        return Err(err);
    }
    let fp = fp.unwrap();
    let mut lines: Vec<String> = Vec::new();
    for i in io::BufReader::new(&fp).lines() {
        if let Err(err) = i {
            return Err(err);
        }
        lines.push(i.unwrap());
    }
    return Ok(lines);
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.

    let seq1_len = seq1.len();
    let seq2_len = seq2.len();

    let mut c = Grid::new(seq1_len + 1, seq2_len + 1);
    for i in 0..=seq1_len {
        c.set(i, 0, 0).unwrap();
    }
    for i in 0..=seq2_len {
        c.set(0, i, 0).unwrap();
    }

    for i in 0..seq1_len {
        for j in 0..seq2_len {
            if seq1[i] == seq2[j] {
                c.set(i + 1, j + 1, c.get(i, j).unwrap() + 1).unwrap();
            } else {
                let max = if c.get(i + 1, j).unwrap() > c.get(i, j + 1).unwrap() {
                    c.get(i + 1, j).unwrap()
                } else {
                    c.get(i, j + 1).unwrap()
                };
                c.set(i + 1, j + 1, max).unwrap();
            }
        }
    }
    return c;
}

fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        print_diff(lcs_table, lines1, lines2, i - 1, j - 1);
        print!("  {}\n", lines1[i - 1]);
    } else if j > 0
        && (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i - 1, j).unwrap())
    {
        print_diff(lcs_table, lines1, lines2, i, j - 1);
        print!(">  {}\n", lines2[j - 1]);
    } else if i > 0
        && (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i - 1, j).unwrap())
    {
        print_diff(lcs_table, lines1, lines2, i - 1, j);
        print!("<  {}\n", lines1[i - 1]);
    } else {
        print!("");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let lines1 = read_file_lines(filename1);
    let lines2 = read_file_lines(filename2);

    if let Err(err) = lines1.as_ref() {
        println!("Error reading file {}: {}", filename1, err);
        process::exit(1);
    }

    if let Err(err) = lines2.as_ref() {
        println!("Error reading file {}: {}", filename2, err);
        process::exit(1);
    }

    let lines1 = lines1.unwrap();
    let lines2 = lines2.unwrap();

    let lcs_table = lcs(&lines1, &lines2);
    print_diff(&lcs_table, &lines1, &lines2, lines1.len(), lines2.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
