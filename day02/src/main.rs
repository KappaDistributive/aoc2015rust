use std::num::ParseIntError;


const INPUT: &str = include_str!("../input.txt");


struct Present {
    dimensions: Vec<u32>,
}

impl Present {
    fn smallest_side(&self) -> u32 {
        let mut temp = self.dimensions.clone();
        temp.sort();
        temp[0] * temp[1]
    }

    fn shortest_perimeter(&self) -> u32 {
        let mut temp = self.dimensions.clone();
        temp.sort();
        2 * (temp[0] + temp[1])
    }

    fn surface_area(&self) -> u32 {
        2 * (self.dimensions[0] * self.dimensions[1]
             + self.dimensions[0] * self.dimensions[2]
             + self.dimensions[1] * self.dimensions[2])
    }

    fn required_paper(&self) -> u32 {
        self.surface_area() + self.smallest_side()
    }

    fn volume(&self) -> u32 {
        let mut v: u32 = 1;
        for d in self.dimensions.clone() {
            v *= d;
        }
        v
    }

    fn required_ribon(&self) -> u32 {
        self.shortest_perimeter() + self.volume()
    }
}

fn solve_part_1(data: &Vec<Present>) -> u32 {
    data.iter().map(|p| p.required_paper()).sum::<u32>()
}

fn solve_part_2(data: &Vec<Present>) -> u32 {
    data.iter().map(|p| p.required_ribon()).sum::<u32>()
}

fn format_input(input_str: &str) -> Result<Vec<Present>, ParseIntError> {
    let mut presents: Vec<Present> = Vec::new();
    for line in input_str.lines() {
        let dimensions = line.trim()
            .split('x')
            .map(|d| d.parse())
            .collect::<Result<Vec<_>, ParseIntError>>()?;
        presents.push(Present{dimensions});
    }
    Ok(presents)
}

fn main() -> Result<(), ParseIntError> {
    let data = format_input(INPUT)?;

    println!("Answer part 1: {}", solve_part_1(&data));
    println!("Answer part 2: {}", solve_part_2(&data));
    
    Ok(())
}
