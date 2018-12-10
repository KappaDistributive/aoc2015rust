extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

struct Gate {
    type: str,
    data:
}

fn format_input(input_str: &str) {
    lazy_static!{
        static ref RE_NOT: Regex = Regex::new(r"NOT ([a-z]+) -> ([a-z]+)").unwrap();
        static ref RE_OR: Regex = Regex::new(r"([a-z0-9]+) OR ([a-z0-9]+) -> ([a-z]+)").unwrap();
    }
    for line in input_str.trim().lines() {
        let mut temp: Vec<&str> = Vec::new();
        match RE_NOT.captures(line) {
            Some(caps) => {
                temp.push(caps.get(1).map_or("", |p| p.as_str()));
                temp.push(caps.get(2).map_or("", |p| p.as_str()));
            }
            None => {
                continue;
            }
        }
        match RE_OR.captures(line) {
            Some(caps) => {
                temp.push(caps.get(1).map_or("", |p| p.as_str()));
                temp.push(caps.get(2).map_or("", |p| p.as_str()));
                temp.push(caps.get(3).map_or("", |p| p.as_str()));
            }
            None => {
                continue;
            }
        }

        println!("{} {}", temp[0], temp[1]);
    }
}

fn main() {
    format_input(INPUT);
}
