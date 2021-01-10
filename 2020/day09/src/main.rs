use circular_queue::CircularQueue;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let t : u64 = 530627549;

    let numbers : Vec<u64> = r.lines().map(|l| l.unwrap().parse().unwrap()).collect();

    for (pos, e) in numbers.iter().enumerate() {
        let mut x = pos + 1;
        let mut counter = *e;
        loop {
            if counter > t || x == numbers.len() {
                break;
            }
            if counter == t {
                let max = numbers[pos..x].iter().max().unwrap();
                let min = numbers[pos..x].iter().min().unwrap();
                println!("Range {} to {}: {}+{}={}", pos, x, min, max, min+max);
            }
            counter += *numbers.get(x).unwrap();
            x += 1;
        }
    }
}
