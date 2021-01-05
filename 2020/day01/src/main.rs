use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let lines: Vec<i64> = r.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    for l1 in &lines {
        for l2 in &lines {
            for l3 in &lines {
                if l1 + l2 + l3 == 2020 {
                    println!("{}", l1 * l2 * l3);
                }
            }
        }
    }
}
