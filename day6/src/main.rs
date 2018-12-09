extern crate regex;
#[macro_use]
extern crate lazy_static;


use regex::Regex;
const INPUT: &str = include_str!("../input.txt");

type Pair = (usize,usize);

struct Instruction {
    // 0 == turn off, 1 == turn on, 2 == toggle
    action: u8,
    start: Pair,
    end: Pair,
}

fn get_action(instruction: &str) -> u8 {
    let i: String = String::from(instruction);
    if i.contains("off") {
        return 0;
    }
    if i.contains("on") {
        return 1;
    }
    if i.contains("toggle") {
        return 2;
    }
    unreachable!();
}

fn get_coords(instruction: &str) -> (Pair,Pair) {
    lazy_static!  {
        static ref RE: Regex = Regex::new(r"\D*([0-9]*),([0-9]*)\D*([0-9]*),([0-9]*)").unwrap();
    }
    let caps = RE.captures(instruction).unwrap();

    (
        (caps.get(1).map_or(0usize, |m| m.as_str().parse::<usize>().unwrap()),
      caps.get(2).map_or(0usize, |m| m.as_str().parse::<usize>().unwrap()))
     ,
        (caps.get(3).map_or(0usize, |m| m.as_str().parse::<usize>().unwrap()),
         caps.get(4).map_or(0usize, |m| m.as_str().parse::<usize>().unwrap()))
    )
}

fn format_input(input_str: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input_str.trim().lines() {
        let action: u8 = get_action(&line);
        let (start,end) = get_coords(&line);
        instructions.push(Instruction{ action, start, end});
    }
    instructions
}

fn solve_part_1(input: &Vec<Instruction>) -> u32 {
    let mut lit = 0u32;
    let mut grid = vec![vec![false; 1000];1000];
    
    for i in input {        
        for x in i.start.0..=i.end.0 {
            for y in i.start.1..=i.end.1 {
                if i.action == 0 {
                    grid[x][y] = false;
                }
                if i.action == 1 {
                    grid[x][y] = true;
                }
                if i.action == 2 {
                    grid[x][y] = !grid[x][y];
                }
            }
        }
    }
    for x in 0..1000 {
        for y in 0..1000 {
            if grid[x][y] {
                lit +=1;
            }
        }
    }
    lit
}

fn solve_part_2(input: &Vec<Instruction>) -> u32 {
    let mut lit = 0u32;
    let mut grid = vec![vec![0u32; 1000];1000];
    
    for i in input {        
        for x in i.start.0..=i.end.0 {
            for y in i.start.1..=i.end.1 {
                if i.action == 0  && grid[x][y] > 0 {
                    grid[x][y] -= 1;
                }
                if i.action == 1 {
                    grid[x][y] += 1;
                }
                if i.action == 2 {
                    grid[x][y] += 2;
                }
            }
        }
    }
    for x in 0..1000 {
        for y in 0..1000 {
            lit += grid[x][y];
        }
    }
    lit
}

fn main() {
    let input: Vec<Instruction> = format_input(INPUT);
    
    println!("Answer part 1: {}", solve_part_1(&input));
    println!("Answer part 2: {}", solve_part_2(&input));

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_action() {
        assert_eq!(get_action(&"turn on 252,603 through 349,655"),1);
        assert_eq!(get_action(&"toggle 341,780 through 861,813"),2);
        assert_eq!(get_action(&"turn off 102,527 through 650,747"),0);
    }

    #[test]
    fn test_get_coords() {
        assert_eq!(get_coords(&"turn on 252,603 through 349,655"),((252,603),(349,655)));
        assert_eq!(get_coords(&"toggle 341,780 through 861,813"),((341,780),(861,813)));
        assert_eq!(get_coords(&"turn off 102,527 through 650,747"),((102,527),(650,747)));
    }
}
