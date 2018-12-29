use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

type Point = (i32,i32);
fn format_input(input_str: &str) -> Vec<char>{
    input_str.trim().chars().collect()
}

fn solve_part_1(data: &Vec<char>) -> u32 {
    let mut pos: Point = (0,0);
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(pos);
    let mut unique = 1u32;
    let mut index = 0u32;
    for d in data {
        if *d == '>' {
            pos.0 += 1;
        }
        if *d == '<' {
            pos.0 -= 1;
        }
        if *d == 'v' {
            pos.1 += 1;
        }
        if *d == '^' {
            pos.1 -= 1;
        }
        if !visited.contains(&pos) {
            unique +=1;
        }
        visited.insert(pos);
      }
    unique
}

fn solve_part_2(data: &Vec<char>) -> u32 {
    let mut pos_santa: Point = (0,0);
    let mut pos_robo: Point = (0,0);
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(pos_santa);
    let mut unique = 1u32;
    let mut index = 0u32;
    for d in data {
        if index % 2 == 0 {
            if *d == '>' {
                pos_santa.0 += 1;
            }
            if *d == '<' {
                pos_santa.0 -= 1;
            }
            if *d == 'v' {
                pos_santa.1 += 1;
            }
            if *d == '^' {
                pos_santa.1 -= 1;
            }
            if !visited.contains(&pos_santa) {
                unique +=1;
            }
            visited.insert(pos_santa);
        }
        else {
            if *d == '>' {
                pos_robo.0 += 1;
            }
            if *d == '<' {
                pos_robo.0 -= 1;
            }
            if *d == 'v' {
                pos_robo.1 += 1;
            }
            if *d == '^' {
                pos_robo.1 -= 1;
            }
            if !visited.contains(&pos_robo) {
                unique +=1;
            }
            visited.insert(pos_robo);
        }
        index += 1;
    }
    unique
}

fn main() {
    let input = format_input(INPUT);
    println!("Answer part 1: {}", solve_part_1(&input));
    println!("Answer part 2: {}", solve_part_2(&input));
}
