use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");
const EGGNOG: usize = 150;

fn format_input(input: &str) -> Vec<usize> {
    let mut containers: Vec<usize> = Vec::new();
    for line in input.trim().lines() {
        containers.push(line.parse::<usize>().unwrap());
    }
    containers
}

fn solve_rec(containers: &Vec<usize>, index: usize, eggnog: usize, combinations: &mut usize) {
    if index == containers.len() {
        if eggnog == 0 {
            *combinations += 1;
        }
        return;
    }

    if eggnog >= containers[index] {
        solve_rec(
            containers,
            index + 1,
            eggnog - containers[index],
            combinations,
        );
    }
    solve_rec(containers, index + 1, eggnog, combinations);
}

fn solve_rec_2(
    containers: &Vec<usize>,
    index: usize,
    taken: usize,
    eggnog: usize,
    combinations: &mut HashMap<usize, usize>,
) {
    if index == containers.len() {
        if eggnog == 0 {
            *combinations.entry(taken).or_insert(0) += 1;
        }
        return;
    }

    if eggnog >= containers[index] {
        solve_rec_2(
            containers,
            index + 1,
            taken + 1,
            eggnog - containers[index],
            combinations,
        );
    }
    solve_rec_2(containers, index + 1, taken, eggnog, combinations);
}

fn solve_part_1(input: &str) -> usize {
    let containers: Vec<usize> = format_input(input);
    let mut combinations: usize = 0;
    solve_rec(&containers, 0, EGGNOG, &mut combinations);
    combinations
}

fn solve_part_2(input: &str) -> usize {
    let containers: Vec<usize> = format_input(input);
    let mut combinations: HashMap<usize, usize> = HashMap::new();
    solve_rec_2(&containers, 0, 0, EGGNOG, &mut combinations);
    let min = combinations.keys().min().unwrap();
    *combinations.get(&min).unwrap()
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
