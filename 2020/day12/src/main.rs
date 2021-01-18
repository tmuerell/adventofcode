use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

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

    let mut x : i64 = 0;
    let mut y : i64  = 0;
    let mut wpx : i64  = 10;
    let mut wpy : i64  = 1;

    for step in s {
        match step.moove {
            Move::East => wpx+=step.arg,
            Move::North => wpy+=step.arg,
            Move::West => wpx-=step.arg,
            Move::South => wpy-=step.arg,
            Move::Left => {
                match step.arg {
                    90 => {
                        let x = wpx;
                        let y = wpy;
                        wpx = -y;
                        wpy = x;
                    }
                    180 => {
                        let x = wpx;
                        let y = wpy;
                        wpx = -x;
                        wpy = -y;
                    }
                    270 => {
                        let x = wpx;
                        let y = wpy;
                        wpx = y;
                        wpy = -x;
                    }
                    _ => panic!("not supported")
                }
            },
            Move::Right => {
                match step.arg {
                    90 => {
                        let x = wpx;
                        let y = wpy;
                        wpx = y;
                        wpy = -x;
                    }
                    180 => {
                        let x = wpx;
                        let y = wpy;
                        wpx = -x;
                        wpy = -y;
                    }
                    270 => {
                        let x = wpx;
                        let y = wpy;
                        wpx = -y;
                        wpy = x;
                    }
                    _ => panic!("not supported")
                }
            },
            Move::Forward => {
                    x += wpx*step.arg;
                    y += wpy*step.arg;
            }
        }
    }

    println!("{}", x.abs() + y.abs());
}
