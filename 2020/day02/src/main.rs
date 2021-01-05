use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() {
    let f = File::open("input.txt").unwrap();
    let mut counter = 0;
    let r = BufReader::new(f);
    let re = Regex::new(r"^(\d+)-(\d+) (.): (\w+)$").unwrap();
    for l in r.lines() {
        let l = l.unwrap();
        let c = re.captures(&l).unwrap();
        let min : usize = c[1].parse().unwrap();
        let max : usize = c[2].parse().unwrap();
        let char  = &c[3];
        let word = &c[4];
        if word.chars().nth(min-1).unwrap().to_string() == char && word.chars().nth(max-1).unwrap().to_string() != char {
            counter += 1;
        }
        if word.chars().nth(min-1).unwrap().to_string() != char && word.chars().nth(max-1).unwrap().to_string() == char {
            counter += 1;
        }
    }
    println!("{}", counter);
}
