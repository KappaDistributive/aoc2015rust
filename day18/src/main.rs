const INPUT: &str = include_str!("../input.txt");

struct GameOfLight {
    states: Vec<u8>,
    width: usize,
    height: usize,
}

impl GameOfLight {
    fn new() -> Self {
        GameOfLight::from_size(100, 100)
    }

    fn from_size(width: usize, height: usize) -> Self {
        let states: Vec<u8> = vec![0; 100];

        GameOfLight {
            states,
            width,
            height,
        }
    }

    fn from_data(input: &str) -> Self {
        let mut states: Vec<u8> = Vec::new();
        let height = input.lines().count();
        let mut width: usize = 0;
        for line in input.lines() {
            width = line.chars().count();
            for c in line.chars() {
                match c {
                    '#' => {
                        states.push(1);
                    }
                    '.' => {
                        states.push(0);
                    }
                    _ => panic!("Couldn't parse: {}", c),
                }
            }
        }
        GameOfLight {
            states,
            width,
            height,
        }
    }

    fn step(&mut self) {
        let mut toggle: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get_state(x, y) {
                    // A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
                    0 => {
                        if self.neighbor_count(x, y) == 3 {
                            toggle.push((x, y));
                        }
                    }
                    // A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
                    1 => {
                        if self.neighbor_count(x, y) != 2 && self.neighbor_count(x, y) != 3 {
                            toggle.push((x, y));
                        }
                    }
                    _ => panic!("Invalid state detected at ({},{})", x, y),
                }
            }
        }        
        for (x, y) in toggle {
            self.toggle_state(x, y);
        }
    }

    fn neighbor_count(&self, x: usize, y: usize) -> usize {
        let mut count: usize = 0;
        for j in 0..3 {
            for i in 0..3 {
                if (i != 1 || j != 1)
                    && y + j > 0
                    && y + j <= self.height
                    && x + i > 0
                    && x + i <= self.width
                    && self.get_state((x + i) - 1, (y + j) - 1) == 1
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn light_count(&self) -> usize {
        let mut count: usize = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_state(x, y) == 1 {
                    count += 1;
                }
            }
        }
        count
    }

    fn get_state(&self, x: usize, y: usize) -> u8 {
        self.states[y * self.width + x]
    }

    fn set_state(&mut self, x: usize, y: usize, value: u8) {
        self.states[y * self.width + x] = value;
    }

    fn toggle_state(&mut self, x: usize, y: usize) {
        match self.get_state(x, y) {
            0 | 1 => self.set_state(x, y, 1 - self.get_state(x, y)),
            _ => panic!("Invalid state detected at ({},{})", x, y),
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", if self.get_state(x, y) == 1 { '#' } else { '.' });
            }
            println!();
        }
    }
}

fn solve_part_1(input: &str) -> usize {
    let mut game: GameOfLight = GameOfLight::from_data(input);
    for _ in 0..100 {
        game.step();
    }
    game.light_count()
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));
}
