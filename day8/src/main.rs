
const INPUT: &str = include_str!("../input.txt");
const SAMPLE_INPUT: &str = include_str!("../sample_input.txt");

fn solve_part_1(input: &str) -> u32 {
    let mut escape_hex = 0u32;
    let mut escape_slash = 0u32;
    let mut escape_quote = 0u32;
    let mut number_quotes = 0u32;
    
    for line in input.trim().lines() {
        number_quotes += 2;
        let chars: Vec<char> = line.trim().chars().collect();
        let mut i: usize = 1usize;
        while i < chars.len() { 
            let mut pattern: String = String::new();
            pattern.push(chars[i-1]);
            pattern.push(chars[i]);
            if pattern == String::from("\\x") {
                i += 3;
                escape_hex += 1;
            }
            if pattern == String::from("\\\\") {
                i += 1;
                escape_slash += 1;
            }
            if pattern == String::from("\\\"") {
                i += 1;
                escape_quote += 1;
            }
            i += 1;
        }
    }
    3 * escape_hex + escape_slash + escape_quote + number_quotes
}

fn main() {
    println!("{}", solve_part_1(INPUT));
}
