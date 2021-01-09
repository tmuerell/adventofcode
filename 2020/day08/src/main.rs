use std::fs::File;
use std::io::{BufRead, BufReader};
use strum::{EnumIter, EnumString};
use std::str::FromStr;
use strum::IntoEnumIterator;
use std::collections::HashSet;


#[derive(EnumString, EnumIter, PartialEq, Clone)]
#[strum(serialize_all = "snake_case")]
enum OpCode {
    Acc,
    Jmp,
    Nop
}

#[derive(Clone)]
struct Instruction {
    op : OpCode,
    arg : i32
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_at(s.find(" ").unwrap());

        Ok(Instruction { op: OpCode::from_str(x).unwrap(), arg: y[1..].parse().unwrap() })
    }
}

fn run(prog : &Vec<Instruction>) {
    let mut tracker : HashSet<usize> = HashSet::new();
    let mut acc : i32 = 0;
    let mut pointer : usize = 0;
    loop {
        if tracker.contains(&pointer) {
            //println!("Endless loop detected. Acc: {}", acc);
            return;
        }
        if pointer == prog.len() {
            println!("Program ended. Acc: {}", acc);
            return;
        }
        tracker.insert(pointer);
        let i = prog.get(pointer).unwrap();
        match i.op {
            OpCode::Acc => {
                acc += i.arg;
                pointer += 1
            },
            OpCode::Nop => {
                pointer += 1;
            },
            OpCode::Jmp => {
                if i.arg.is_negative() {
                    pointer -= i.arg.wrapping_abs() as u32 as usize
                } else {
                    pointer += i.arg as usize
                }
            }
        }
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let prog : Vec<Instruction> = r.lines().map(|l| Instruction::from_str(&l.unwrap()).unwrap() ).collect();

    for (pos, e) in prog.iter().enumerate() {
        //println!("Pos: {}", pos);
        if e.op == OpCode::Jmp {
            let mut x = prog.clone();
            x.splice(pos..pos+1, vec![ Instruction { op: OpCode::Nop, arg: e.arg }]);
            run(&x);
        }
        if e.op == OpCode::Nop {
            let mut x = prog.clone();
            x.splice(pos..pos+1, vec![ Instruction { op: OpCode::Jmp, arg: e.arg }]);
            run(&x);
        }
    }

    run(&prog);
}
