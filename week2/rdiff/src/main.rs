use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
#[allow(unused)]
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut res = Vec::new();
    for line in std::io::BufReader::new(file).lines() {
        let str = line?;
        res.push(str);
    }
    Ok(res)
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    let mut res = Grid::new(seq1.len() + 1, seq2.len() + 1);
    for i in 0..=seq1.len() {
        res.set(i, 0, 0);
    }
    for i in 0..=seq2.len() {
        res.set(0, i, 0);
    }
    for i in 0..seq1.len() {
        for j in 0..seq2.len() {
            if seq1[i] == seq2[j] {
                res.set(i+1, j+1, res.get(i, j).unwrap()+1);
            } else {
                // 有std::max 吗？很需要！
                res.set(i+1, j+1, 
                    if res.get(i+1, j).unwrap() > res.get(i, j+1).unwrap()
                     {res.get(i+1, j).unwrap()} 
                     else {res.get(i, j+1).unwrap()});
            }
        }
    }
    res
    // Be sure to delete the #[allow(unused)] line above
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn print_diff(lcs_table: &Grid, lines1: &Vec<String>, lines2: &Vec<String>, i: usize, j: usize) {
    // 递归角度,越靠后的行越晚打印,所以是后序
    
    // 如果两个文本当前行一样,则这行入栈,各自行数都-1，处理前一行
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        print_diff(lcs_table, lines1, lines2, i-1, j-1);
        println!("  {}" , lines1[i-1]);
        // 文本2还有内容, 如果文本1没有内容 , 或者dp[i][j-1]的最长子序列比dp[i-1][1]长（说明i对应的行更重要，文本2的j行删了也没关系,
        // 对应文本2比文本1多的内容
    } else  if j > 0 && (i == 0 || lcs_table.get(i, j - 1).unwrap() >= lcs_table.get(i-1, j).unwrap()) {
        print_diff(lcs_table, lines1, lines2, i, j-1);
        println!(">  {}" , lines2[j-1]);
        // 上个分支反过来，文本1比文本2多的内容
    }  else if i > 0 && (j == 0 || lcs_table.get(i, j - 1).unwrap() < lcs_table.get(i-1, j).unwrap()) {
        print_diff(lcs_table, lines1, lines2, i-1, j);
        println!("<  {}" , lines1[i-1]);
    } else {
        println!("");
    }

    // Be sure to delete the #[allow(unused)] line above
}

#[allow(unused)] // TODO: delete this line when you implement this function
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];
    let vec1 = read_file_lines(filename1).expect("we fucked up");
    let vec2 = read_file_lines(filename2).expect("we fucked up");
    let grid = lcs(&vec1, &vec2);
    print_diff(&grid, &vec1, &vec2, vec1.len(), vec2.len());
    // Be sure to delete the #[allow(unused)] line above
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
