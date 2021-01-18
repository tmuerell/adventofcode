use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum LookDirection {
    North,
    South,
    East,
    West
}

#[derive(Clone, Debug, PartialEq)]
enum Move {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward
}

impl FromStr for Move {
    type Err = ();

    fn from_str(input: &str) -> Result<Move, Self::Err> {
        match input {
            "N" => Ok(Move::North),
            "S" => Ok(Move::South),
            "E" => Ok(Move::East),
            "W" => Ok(Move::West),
            "L" => Ok(Move::Left),
            "R" => Ok(Move::Right),
            "F" => Ok(Move::Forward),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
struct Step {
    moove : Move,
    arg : i64
}

impl FromStr for Step {
    type Err = ();

    fn from_str(input: &str) -> Result<Step, Self::Err> {
        Ok(
            Step {
                moove : Move::from_str(&input[0..1]).unwrap(),
                arg : input[1..].parse().unwrap()
            }
        )
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);

    let s : Vec<Step> = r.lines().map(|s| Step::from_str(&s.unwrap()).unwrap()).collect();

    let mut dir = LookDirection::East;
    let mut x = 0;
    let mut y = 0;

    for step in s {
        match step.moove {
            Move::East => x+=step.arg,
            Move::North => y+=step.arg,
            Move::West => x-=step.arg,
            Move::South => y-=step.arg,
            Move::Left => {
                match (dir, step.arg) {
                    (LookDirection::East, 90) => dir = LookDirection::North,
                    (LookDirection::East, 180) => dir = LookDirection::West,
                    (LookDirection::East, 270) => dir = LookDirection::South,
                    (LookDirection::North, 90) => dir = LookDirection::West,
                    (LookDirection::North, 180) => dir = LookDirection::South,
                    (LookDirection::North, 270) => dir = LookDirection::East,
                    (LookDirection::West, 90) => dir = LookDirection::South,
                    (LookDirection::West, 180) => dir = LookDirection::East,
                    (LookDirection::West, 270) => dir = LookDirection::North,
                    (LookDirection::South, 90) => dir = LookDirection::East,
                    (LookDirection::South, 180) => dir = LookDirection::North,
                    (LookDirection::South, 270) => dir = LookDirection::West,
                    (_, _) => panic!("not supported")
                }
            },
            Move::Right => {
                match (dir, step.arg) {
                    (LookDirection::East, 90) => dir = LookDirection::South,
                    (LookDirection::East, 180) => dir = LookDirection::West,
                    (LookDirection::East, 270) => dir = LookDirection::North,
                    (LookDirection::North, 90) => dir = LookDirection::East,
                    (LookDirection::North, 180) => dir = LookDirection::South,
                    (LookDirection::North, 270) => dir = LookDirection::West,
                    (LookDirection::West, 90) => dir = LookDirection::North,
                    (LookDirection::West, 180) => dir = LookDirection::East,
                    (LookDirection::West, 270) => dir = LookDirection::South,
                    (LookDirection::South, 90) => dir = LookDirection::West,
                    (LookDirection::South, 180) => dir = LookDirection::North,
                    (LookDirection::South, 270) => dir = LookDirection::East,
                    (_, _) => panic!("not supported")
                }
            },
            Move::Forward => {
                match dir {
                    LookDirection::East => x += step.arg,
                    LookDirection::North => y += step.arg,
                    LookDirection::West => x -= step.arg,
                    LookDirection::South => y -= step.arg
                }
            }
        }
    }

    println!("{}", x.abs() + y.abs());
}
