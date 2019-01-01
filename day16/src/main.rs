#[macro_use]
extern crate lazy_static;
use regex::Regex;

const INPUT: &str = include_str!("../input.txt");

struct Aunt {
    id: usize,
    cars: Option<usize>,
    cats: Option<usize>,
    children: Option<usize>,
    goldfish: Option<usize>,
    perfumes: Option<usize>,
    trees: Option<usize>,
    samoyeds: Option<usize>,
    pomeranians: Option<usize>,
    akitas: Option<usize>,
    vizslas: Option<usize>,
}

impl Aunt {
    fn from_str(input: &str) -> Self {
        lazy_static! {
            static ref RE_ID: Regex = Regex::new(r"Sue (?P<value>[0-9]+):").unwrap();
            static ref RE_CARS: Regex = Regex::new(r"cars: (?P<value>[0-9]+)").unwrap();
            static ref RE_CATS: Regex = Regex::new(r"cats: (?P<value>[0-9]+)").unwrap();
            static ref RE_CHILDREN: Regex = Regex::new(r"children: (?P<value>[0-9]+)").unwrap();
            static ref RE_GOLDFISH: Regex = Regex::new(r"goldfish: (?P<value>[0-9]+)").unwrap();
            static ref RE_PERFUMES: Regex = Regex::new(r"perfumes: (?P<value>[0-9]+)").unwrap();
            static ref RE_TREES: Regex = Regex::new(r"trees: (?P<value>[0-9]+)").unwrap();
            static ref RE_SAMOYEDS: Regex = Regex::new(r"samoyeds: (?P<value>[0-9]+)").unwrap();
            static ref RE_POMERANIANS: Regex =
                Regex::new(r"pomeranians: (?P<value>[0-9]+)").unwrap();
            static ref RE_AKITAS: Regex = Regex::new(r"akitas: (?P<value>[0-9]+)").unwrap();
            static ref RE_VIZSLAS: Regex = Regex::new(r"vizslas: (?P<value>[0-9]+)").unwrap();
        }
        
        Aunt {
            id: extract_value(input, &RE_ID).unwrap(),
            cars: extract_value(input, &RE_CARS),
            cats: extract_value(input, &RE_CATS),
            children: extract_value(input, &RE_CHILDREN),
            goldfish: extract_value(input, &RE_GOLDFISH),
            perfumes: extract_value(input, &RE_PERFUMES),
            trees: extract_value(input, &RE_TREES),
            samoyeds: extract_value(input, &RE_SAMOYEDS),
            pomeranians: extract_value(input, &RE_POMERANIANS),
            akitas: extract_value(input, &RE_AKITAS),
            vizslas: extract_value(input, &RE_VIZSLAS),
        }
    }

    fn is_compatible(&self, candidate: &Aunt) -> bool {
        candidate.cars.unwrap_or(self.cars.unwrap()) == self.cars.unwrap() &&
            candidate.cats.unwrap_or(self.cats.unwrap()) == self.cats.unwrap() &&
            candidate.children.unwrap_or(self.children.unwrap()) == self.children.unwrap() &&
            candidate.goldfish.unwrap_or(self.goldfish.unwrap()) == self.goldfish.unwrap() &&
            candidate.perfumes.unwrap_or(self.perfumes.unwrap()) == self.perfumes.unwrap() &&
            candidate.trees.unwrap_or(self.trees.unwrap()) == self.trees.unwrap() &&
            candidate.samoyeds.unwrap_or(self.samoyeds.unwrap()) == self.samoyeds.unwrap() &&
            candidate.pomeranians.unwrap_or(self.pomeranians.unwrap()) == self.pomeranians.unwrap() &&
            candidate.akitas.unwrap_or(self.akitas.unwrap()) == self.akitas.unwrap() &&
            candidate.vizslas.unwrap_or(self.vizslas.unwrap()) == self.vizslas.unwrap()
    }
}

fn extract_value(input: &str, re: &Regex) -> Option<usize> {
    match re.captures(input) {
        Some(cap) => Some(
            cap.name("value")
                .map_or(0usize, |m| m.as_str().parse::<usize>().unwrap())
        ),
        None => None,
    }
}

fn solve_part_1(input_str: &str, aunt: Aunt) -> usize {
    let mut aunts: Vec<Aunt> = Vec::new();
    let mut eliminated: Vec<bool> = Vec::new();
    for line in input_str.trim().lines() {
        aunts.push(Aunt::from_str(line));
    }

    for i in 0..aunts.len() {
        let compatible = aunt.is_compatible(&aunts[i]);
        eliminated.push(!compatible);
        if compatible {
            return aunts[i].id;
        }
    }
    unreachable!("Couldn't find a match!");
}

fn main() {
    let ticker_tape: Aunt = Aunt {
        id: 0,
        cars: Some(2),
        cats: Some(7),
        children: Some(3),
        goldfish: Some(5),
        perfumes: Some(1),
        trees: Some(3),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
    };    
    println!("Answer part 1: {}", solve_part_1(INPUT, ticker_tape));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aunt_from_str() {
        let result: Aunt = Aunt::from_str(&"Sue 1: children: 1, cars: 8, vizslas: 7");
        let expected: Aunt = Aunt {
            id: 1,
            cars: Some(8),
            cats: None,
            children: Some(1),
            goldfish: None,
            perfumes: None,
            trees: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: Some(7),
        };
    }
}
