#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashSet;

const RULES: &str = include_str!("../input.txt");
const MOLECULE: &str = &"CRnCaSiRnBSiRnFArTiBPTiTiBFArPBCaSiThSiRnTiBPBPMgArCaSiRnTiMgArCaSiThCaSiRnFArRnSiRnFArTiTiBFArCaCaSiRnSiThCaCaSiRnMgArFYSiRnFYCaFArSiThCaSiThPBPTiMgArCaPRnSiAlArPBCaCaSiRnFYSiThCaRnFArArCaCaSiRnPBSiRnFArMgYCaCaCaCaSiThCaCaSiAlArCaCaSiRnPBSiAlArBCaCaCaCaSiThCaPBSiThPBPBCaSiRnFYFArSiThCaSiRnFArBCaCaSiRnFYFArSiThCaPBSiThCaSiRnPMgArRnFArPTiBCaPRnFArCaCaCaCaSiRnCaCaSiRnFYFArFArBCaSiThFArThSiThSiRnTiRnPMgArFArCaSiThCaPBCaSiRnBFArCaCaPRnCaCaPMgArSiRnFYFArCaSiThRnPBPMgAr";

fn parse_rules(input: &str) -> HashSet<(String, String)> {
    lazy_static! {
        static ref RE_RULES: Regex =
            Regex::new(r"(?P<educt>[a-zA-Z]+)[^a-zA-z]+(?P<product>[a-zA-Z]+)").unwrap();
    }
    let mut rules: HashSet<(String, String)> = HashSet::new();

    for line in input.lines() {
        match RE_RULES.captures(line) {
            Some(cap) => {
                let educt: String = String::from(cap.name("educt").map_or("", |m| m.as_str()));
                let product: String = String::from(cap.name("product").map_or("", |m| m.as_str()));
                rules.insert((educt, product));
            }
            None => panic!("Couldn't parse rule: {}", line),
        }
    }
    rules
}

fn solve_part_1(input_rules: &str, input_molecule: &str) -> usize {
    let rules: HashSet<(String, String)> = parse_rules(input_rules);
    let mut results: HashSet<String> = HashSet::new();
    let chars: Vec<char> = input_molecule.chars().collect();
    for (educt, product) in rules.iter() {
        match educt.chars().count() {
            1 => {
                for i in 0..chars.len() {
                    if chars[i].to_string() == *educt {
                        let mut temp: String = String::from(&input_molecule[0..i]);
                        temp.push_str(&product);
                        temp.push_str(&input_molecule[i + 1..]);
                        results.insert(temp);
                    }
                }
            }
            2 => {
                for i in 1..chars.len() {
                    let mut pattern: String = String::new();
                    pattern.push(chars[i - 1]);
                    pattern.push(chars[i]);
                    if pattern == *educt {
                        let mut temp: String = String::from(&input_molecule[0..i - 1]);
                        temp.push_str(&product);
                        temp.push_str(&input_molecule[i + 1..]);
                        results.insert(temp);
                    }
                }
            }
            _ => {
                panic!("Unrecognized rule: {} => {}", educt, product);
            }
        }
    }
    results.len()
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(RULES, MOLECULE));
}
