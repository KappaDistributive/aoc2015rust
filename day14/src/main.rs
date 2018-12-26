#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;
use std::cmp::{max, min};

const INPUT: &str = include_str!("../input.txt");

struct Reindeer {
    name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

impl Reindeer {
    fn from_str(input: &str) -> Self {
        lazy_static! {
            static ref RE_REINDEER: Regex = Regex::new(r"(?P<name>[a-zA-Z]+)[^0-9]+(?P<speed>[0-9]+)[^0-9]+(?P<duration>[0-9]+)[^0-9]+(?P<rest>[0-9]+)[^0-9]+").unwrap();
        }
        match RE_REINDEER.captures(input) {
            Some(cap) => {
                let name = cap
                    .name("name")
                    .map_or(String::from(""), |m| String::from(m.as_str()));
                let speed = cap
                    .name("speed")
                    .map_or(0u32, |m| m.as_str().parse::<u32>().unwrap());
                let duration = cap
                    .name("duration")
                    .map_or(0u32, |m| m.as_str().parse::<u32>().unwrap());
                let rest = cap
                    .name("rest")
                    .map_or(0u32, |m| m.as_str().parse::<u32>().unwrap());

                Reindeer {
                    name,
                    speed,
                    duration,
                    rest,
                }
            }
            None => {
                panic!("Couldn't parse reindeer: {}", input);
            }
        }
    }

    fn distance_travelled(&self, seconds: u32) -> u32 {
        // distance travelled during full cycles
        ((seconds / (self.duration + self.rest)) * self.duration * self.speed)
            // distance travelled in the remaining time
            + min(seconds % (self.duration + self.rest), self.duration) * self.speed
    }
}

fn format_input(input_str: &str) -> Vec<Reindeer> {
    let mut result: Vec<Reindeer> = Vec::new();
    for line in input_str.lines() {
        result.push(Reindeer::from_str(line));
    }
    result
}

fn solve_part_1(input_str: &str) -> u32 {
    let reindeers: Vec<Reindeer> = format_input(input_str);
    let mut results: Vec<(String, u32)> = Vec::new();
    let mut result: u32 = 0;

    for reindeer in reindeers {
        results.push((reindeer.name.clone(), reindeer.distance_travelled(2503)));
    }
    for (_, dst) in results {
        if dst > result {
            result = dst;
        }
    }
    result
}

fn solve_part_2(input_str: &str) -> u32 {
    let reindeers: Vec<Reindeer> = format_input(input_str);
    let mut scores: Vec<u32> = Vec::new();
    for _ in 0..reindeers.len() {
        scores.push(0);
    }
    for t in 1..=2503 {
        let mut distance: Vec<u32> = Vec::new();
        let mut temp: u32 = 0u32;
        for r in 0..reindeers.len() {
            let dst: u32 = reindeers[r].distance_travelled(t);
            distance.push(dst);
            temp = max(temp, dst);
        }
        for r in 0..reindeers.len() {
            if distance[r] == temp {
                scores[r] += 1;
            }
        }
    }
    *scores.iter().max().unwrap()
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
