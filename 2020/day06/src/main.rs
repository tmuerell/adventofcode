use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut answers = HashSet::new();
    let mut first = true;
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut counter = 0;
    for l in r.lines() {
        let l = l.unwrap();
        if l.is_empty() {
            counter += answers.len();
            answers.clear();
            first = true;
        } else {
            if first {
                for c in l.chars() {
                    answers.insert(c);
                }
                first = false;
            } else {
                let mut x = HashSet::new();
                for c in l.chars() {
                    x.insert(c);
                }
                answers = answers.intersection(&x).copied().collect();
            }
        }
    }
    counter += answers.len();

    println!("{}", counter);
}
