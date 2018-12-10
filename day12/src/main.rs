extern crate regex;
#[macro_use]
extern crate lazy_static;

use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    status: String,
    content: String
}

fn solve_part_1(input_str: &str) -> i32 {
    let mut result: i32 = 0;
    lazy_static!{
        static ref RE: Regex = Regex::new(r"(-?[0-9]+)").unwrap();
    }
    RE.captures_iter(input_str).map(|c| c[0].parse::<i32>().unwrap()).sum()
}


fn main() {
    println!("{}",solve_part_1(INPUT));
}
