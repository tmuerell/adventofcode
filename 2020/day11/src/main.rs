use std::convert::TryInto;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
enum State {
    Floor,
    OccupiedSeat,
    EmptySeat,
}

impl FromStr for State {
    type Err = ();

    fn from_str(input: &str) -> Result<State, Self::Err> {
        match input {
            "L" => Ok(State::EmptySeat),
            "#" => Ok(State::OccupiedSeat),
            "." => Ok(State::Floor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Area {
    places: Vec<Vec<State>>,
}

impl Area {
    fn rows(&self) -> i64 {
        self.places.len().try_into().unwrap()
    }

    fn columns(&self) -> i64 {
        self.places[0].len().try_into().unwrap()
    }

    fn seat(&self, row: i64, col: i64) -> Option<State> {
        if row < 0 || col < 0 {
            None
        } else {
            if row >= self.rows() || col >= self.columns() {
                None
            } else {
                let x: usize = row.try_into().unwrap();
                let y: usize = col.try_into().unwrap();
                Some(self.places[x][y].clone())
            }
        }
    }

    fn sees_occupied(&self, row: i64, col: i64, rf : i64, cf : i64) -> bool {
        let mut r = row;
        let mut c = col;
        loop {
            r += rf;
            c += cf;
            match self.seat(r, c) {
                None => break,
                Some(s) if s == State::OccupiedSeat => return true,
                Some(s) if s == State::EmptySeat => return false,
                Some(_) => {}
            }
        }
        return false;
    }

    fn count_occupied(&self, row: i64, col: i64) -> usize {
        let mut c = 0;
        if self.sees_occupied(row, col, -1, -1) {
            c += 1;
        }
        if self.sees_occupied(row, col, -1, 0) {
            c += 1;
        }
        if self.sees_occupied(row, col, -1, 1) {
            c += 1;
        }
        if self.sees_occupied(row, col, 0, -1) {
            c += 1;
        }
        if self.sees_occupied(row, col, 0, 1) {
            c += 1;
        }
        if self.sees_occupied(row, col, 1, -1) {
            c += 1;
        }
        if self.sees_occupied(row, col, 1, 0) {
            c += 1;
        }
        if self.sees_occupied(row, col, 1, 1) {
            c += 1;
        }
        c
    }

    fn set(&mut self, row: i64, col: i64, state: State) {
        let x: usize = row.try_into().unwrap();
        let y: usize = col.try_into().unwrap();
        self.places[x][y] = state;
    }

    fn occupied_seats(&self) -> usize {
        let mut co = 0;
        for r in &self.places {
            for c in r {
                if *c == State::OccupiedSeat {
                    co += 1
                }
            }
        }
        co
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);

    let x: Vec<Vec<State>> = r
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| State::from_str(&c.to_string()).unwrap())
                .collect()
        })
        .collect();

    let mut a = Area { places: x };

    for _ in 0..400 {
        let mut b = a.clone();

        for r in 0..a.rows() {
            for c in 0..a.columns() {
                if a.seat(r, c).unwrap() == State::EmptySeat && a.count_occupied(r, c) == 0 {
                    b.set(r, c, State::OccupiedSeat);
                }
                if a.seat(r, c).unwrap() == State::OccupiedSeat && a.count_occupied(r, c) >= 5 {
                    b.set(r, c, State::EmptySeat);
                }
            }
        }

        if a.occupied_seats() == b.occupied_seats() {
            println!("Fertig");
            break;
        }

        a = b;
    }

    println!("{}", a.occupied_seats());
}
