use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug)]
struct Bag {
    name : String,
    contents : Vec<NumberedBagRef>
}

impl Bag {
    fn from(text : &str) -> Bag {
        lazy_static! {
            // mirrored aqua bags contain 3 dim tomato bags, 1 mirrored crimson bag, 1 wavy silver bag.
            static ref TE: Regex = Regex::new(r"^(\w+ \w+) bags contain (.+)\.$").unwrap();
            static ref TE2: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        }
        match TE.captures(text) {
            Some(caps) => {
                Bag {
                    name: caps[1].to_owned(),
                    contents: TE2.captures_iter(&caps[2]).map(|c| NumberedBagRef { count: c[1].parse().unwrap(), reference: c[2].to_owned() }).collect()
                }
            },
            None => panic!("Invalid bag ref.")
        }
    }
}

#[derive(Debug, Clone)]
struct NumberedBagRef {
    count : usize,
    reference : String
}

fn find(bags : &HashMap<String, Bag>, level : usize, start : &str, needle : &str) -> bool {
    println!("  {} {}", level, start);
    if start == needle {
        return true;
    }
    match bags.get(start) {
        Some(b) => {
            for c in &b.contents {
                if find(bags, level + 1, &c.reference, needle) {
                    return true;
                }
            }
            return false;
        }
        None => false
    }
}

fn count_bags(bags : &HashMap<String, Bag>, name : &str) -> usize {
    match bags.get(name) {
        Some(b) => {
            let x : usize = b.contents.iter().map(|c| c.count * count_bags(bags, &c.reference)).sum();
            x + 1
        },
        None => panic!("Waaaah")
    }
}

fn traverse(name : &str, bags : &HashMap<String, Bag>) -> Vec<NumberedBagRef> {
    match bags.get(name) {
        Some(b) => b.contents.clone(),
        None => panic!("Waaaah")
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut bags = HashMap::new();
    for l in r.lines() {
        let l = l.unwrap();
        let b = Bag::from(&l);
        bags.insert(b.name.clone(), b);
    }
    println!("{}", bags.len());
    let bags = bags;
    let mut counter = count_bags(&bags, "shiny gold");
    println!("{}", counter-1);
}
