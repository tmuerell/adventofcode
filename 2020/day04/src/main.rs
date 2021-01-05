use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use strum::{EnumIter, EnumString};
use std::str::FromStr;
use strum::IntoEnumIterator;
use lazy_static::lazy_static;

#[derive(EnumString, EnumIter, PartialEq)]
#[strum(serialize_all = "snake_case")]
enum FieldType {
    Byr,
    Iyr,
    Eyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Cid
}

struct Field {
    field : FieldType,
    value : String
}

struct Passeport {
    fields : Vec<Field>
}

fn valid_passeport(passeport : &Passeport) -> bool {
    let mut valid = true;
    for ft in FieldType::iter() {
        let c = passeport.fields.iter().filter(|x| x.field == ft ).filter(|x| validate_field(x)).count();
        if c == 0 && ft != FieldType::Cid {
            valid = false;
        }
        if c > 1 {
            valid = false
        }
    }
    valid
}

fn validate_field(field : &Field) -> bool {
    match &field.field {
        FieldType::Byr => {
            match field.value.parse::<usize>() {
                Ok(x) => x >= 1920 && x <= 2002,
                Err(_) => false
            }
        },
        FieldType::Iyr => {
            match field.value.parse::<usize>() {
                Ok(x) => x >= 2010 && x <= 2020,
                Err(_) => false
            }
        },
        FieldType::Eyr => {
            match field.value.parse::<usize>() {
                Ok(x) => x >= 2020 && x <= 2030,
                Err(_) => false
            }
        },
        FieldType::Hgt => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
            }
            match RE.captures(&field.value) {
                None => false,
                Some(caps) => {
                    if &caps[2] == "cm" {
                        match caps[1].parse::<usize>() {
                            Ok(x) => x >= 150 && x <= 193,
                            Err(_) => false
                        }
                    } else
                    if &caps[2] == "in" {
                        match caps[1].parse::<usize>() {
                            Ok(x) => x >= 59 && x <= 76,
                            Err(_) => false
                        }
                    } else {
                        false
                    }
                }
            }

        },
        FieldType::Hcl => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            }
            match RE.captures(&field.value) {
                None => false,
                Some(_) => true
            }
        },
        FieldType::Ecl => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
            }
            match RE.captures(&field.value) {
                None => false,
                Some(_) => true
            }
        },
        FieldType::Pid => {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"^\d{9}$").unwrap();
            }
            match RE.captures(&field.value) {
                None => false,
                Some(_) => true
            }
        },
        FieldType::Cid => true
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let r = BufReader::new(f);
    let mut passeport = Passeport { fields: vec![] };
    let ws = Regex::new(r"(...):(\S+)").expect("Invalid regex");
    let mut counter = 0;
    for l in r.lines() {
        let l = l.unwrap();
        if l.is_empty() {
            if valid_passeport(&passeport) {
                counter += 1
            }
            passeport = Passeport { fields: vec![] };
        } else {
            for cap in ws.captures_iter(&l) {
                passeport.fields.push(Field {
                    field: FieldType::from_str(&cap[1]).unwrap(),
                    value: cap[2].to_string()
                })
            }
        }
    }
    if valid_passeport(&passeport) {
        counter += 1
    }
    println!("{}", counter);

}
