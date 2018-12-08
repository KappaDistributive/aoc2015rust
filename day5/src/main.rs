use std::string::String;
const INPUT: &str = include_str!("../input.txt");
const VOWELS: [char;5] = ['a','e','i','o','u'];

fn is_nice(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let forbidden: [String;4] = [String::from("ab"),String::from("cd"),String::from("pq"),String::from("xy")];
    let mut vowel_counter = 0u32;
    let mut duplicate = false;
    for c in chars.clone() {
        if VOWELS.contains(&c) {
            vowel_counter += 1;
        }
    }
    if vowel_counter < 3 {
        return false;
    }
    for i in 1..input.len() {
        if chars[i-1] == chars[i] {
            duplicate = true;
            break;
        }
    }
    if !duplicate {
        return false;
    }
    for i in 1..input.len() {
        let mut temp: String = String::new();
        temp.push(chars[i-1]);
        temp.push(chars[i]);

        if forbidden.contains(&temp) {
            return false;
        }
    }
    
    true
}

fn is_really_nice(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    let mut gap_pair = false;
    let mut repeat = false;
    for i in 2..input.len() {
        if chars[i-2] == chars[i] {
            gap_pair = true;
            break;
        }
    }
    if !gap_pair {
        return false;
    }
    for i in 1..input.len() {
        let mut pattern: String = String::new();
        let tail: String = String::from(&input[i+1..]);
        pattern.push(chars[i-1]);
        pattern.push(chars[i]);
        
        if tail.contains(&pattern) {
            repeat = true;
            break;
        }
    }
    if !repeat {
        return false;
    }
    true
}

fn solve_part_1() -> u32 {
    let mut nice_counter = 0u32;
    for word in INPUT.lines() {
        if is_nice(word) {
            nice_counter += 1;
        }
    }
    nice_counter
}

fn solve_part_2() -> u32 {
    let mut really_nice_counter = 0u32;
    for word in INPUT.lines() {
        if is_really_nice(word) {
            really_nice_counter += 1;
        }
    }
    really_nice_counter
}

fn main() {

    println!("Answer part 1: {}", solve_part_1());
    println!("Answer part 2: {}", solve_part_2());
    
}
