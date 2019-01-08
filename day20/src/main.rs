const INPUT: usize = 29000000;

fn solve_part_1(input: usize) -> usize {
    let mut houses: Vec<usize> = vec![0; input / 10];
    for i in 1..(input / 10) {
        for j in 1..input / (i * 10) {
            houses[j * i] += i * 10;
        }
        if houses[i] >= input {
            return i;
        }
    }
    0
}

fn solve_part_2(input: usize) -> usize {
    let mut houses: Vec<usize> = vec![0; input / 10];
    for i in 1..(input / 10) {
        for j in 1..=50 {
            if j * i < input / 10 {
                houses[j * i] += i * 11;
            }
        }
        if houses[i] >= input {
            return i;
        }
    }
    0
}
fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
    println!("Answer part 2: {}", solve_part_2(INPUT));
}
