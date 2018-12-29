
const INPUT: &str = include_str!("../input.txt");

fn solve_part_1(input: &str) -> i32 {
    let mut answer = 0i32;
    for c in input.chars() {
        if c == '(' {
            answer += 1;
        }
        if c == ')' {
            answer -= 1;
        }
    }
    answer
}

fn solve_part_2(input: &str) -> i32 {
    let mut floor = 0i32;
    let mut index = 1i32;
    for c in input.chars() {
        if c == '(' {
            floor += 1;
        }
        if c == ')' {
            floor -= 1;
        }
        if floor == -1 {
            return index;
        }
        index += 1; 
    }
    -1
}

fn main() {

    println!("Answer part 1: {}", solve_part_1(INPUT));

    println!("Answer part 2: {}", solve_part_2(INPUT));
    
}
