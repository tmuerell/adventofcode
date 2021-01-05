use std::fs::File;
use std::io::{BufRead, BufReader};

struct Seat {
    spec : String
}

impl Seat {
    fn row(&self) -> u64 {
        let bin : String = self.spec[0..7].chars().map(|x| if x == 'F' { '0' } else { '1' } ).collect();
        u64::from_str_radix(&bin, 2).unwrap()
    }

    fn col(&self) -> u64 {
        let bin : String = self.spec[7..10].chars().map(|x| if x == 'L' { '0' } else { '1' } ).collect();
        u64::from_str_radix(&bin, 2).unwrap()
    }

    fn id(&self) -> u64 {
        self.row() * 8 + self.col()
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let seats : Vec<Seat> = r.lines().map(|l| Seat { spec: l.unwrap() }).collect();

    let m = seats.iter().map(|x| x.id()).max().unwrap();
    for i in 2..m {
        let p = seats.iter().find(|x| x.id() == i-1);
        let c = seats.iter().find(|x| x.id() == i);
        let n = seats.iter().find(|x| x.id() == i+1);
        match (p, c, n) {
            (Some(_), None, Some(_)) => println!("{}", i),
            _ => {}
        }

    }
}

#[cfg(test)]
mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn test_id() {
            let s = Seat { spec: "BFFFBBFRRR".to_string() };
            assert_eq!(s.row(), 70);
            assert_eq!(s.col(), 7);
            assert_eq!(s.id(), 567);
        }
    
}
