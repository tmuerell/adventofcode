use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
enum Tile {
    Tree,
    Open
}

fn get_number_of_trees(map : &Vec<Vec<Tile>>, dx : usize, dy : usize) -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    while y < map.len() - 1 {
        y += dy;
        x += dx;

        x = x % map[y].len();

        if map[y][x] == Tile::Tree {
            trees += 1;
        }

    }

    return trees;
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut map : Vec<Vec<Tile>>= vec![];
    for l in r.lines() {
        let l = l.unwrap();
        map.push(l.chars().map(|c| {
            if c == '#' {
                Tile::Tree
            } else {
                Tile::Open
            }
        }).collect());
    }
    
    let mut prod = 1;

    prod *= get_number_of_trees(&map, 1, 1);
    prod *= get_number_of_trees(&map, 3, 1);
    prod *= get_number_of_trees(&map, 5, 1);
    prod *= get_number_of_trees(&map, 7, 1);
    prod *= get_number_of_trees(&map, 1, 2);

    

    println!("{} trees hit.", prod);
}
