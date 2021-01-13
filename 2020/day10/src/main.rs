use std::fs::File;
use std::io::{BufRead, BufReader};

fn step(jolts : &[u64], cur : u64) -> usize {
    if jolts.len() == 1 {
        1
    } else {
        if jolts.len() == 2 {
            1
        
    } else {
        let mut c = 0;
        if jolts[1] <= cur + 3 {
            c += step(&jolts[1..], jolts[1]);
        }
        if jolts[2] <= cur + 3 {
            c += step(&jolts[2..], jolts[2]);
        }
        if jolts.len() > 3 && jolts[3] <= cur + 3 {
            c += step(&jolts[3..], jolts[3]);
        }
        c
    }
}
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);

    let mut jolts : Vec<u64> = r.lines().map(|l| l.unwrap().parse().unwrap()).collect();
    jolts.push(0);

    jolts.sort();

    let min = 0;
    let max = dbg!(jolts.iter().max().unwrap());

    let jolts = &jolts;
    let e = jolts.len();

    let mut prev = 0;
    let mut total = 1;
    for i in 1..e {
        if jolts[i] == jolts[i-1] + 3 {
            //println!("3-gap at {}", i);
            //println!("{}-{}", prev, i);
            println!("{:?}", &jolts[prev..i]);
            total *= dbg!(step(&jolts[prev..i], jolts[prev]));
            prev = i;
        }
    }
    println!("{:?}", &jolts[prev..e]);
    total *= dbg!(step(&jolts[prev..e], jolts[prev]));

    println!("{}", total);
}
