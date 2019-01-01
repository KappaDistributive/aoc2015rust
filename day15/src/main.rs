#[macro_use]
extern crate lazy_static;
use regex::Regex;
use integer_partitions::Partitions;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn from_data(
        name: String,
        capacity: i64,
        durability: i64,
        flavor: i64,
        texture: i64,
        calories: i64,
    ) -> Self {
        Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        }
    }
}

fn format_input(input_str: &str) -> Vec<Ingredient> {
    lazy_static! {
        static ref RE_INGREDIENT: Regex = Regex::new(r"(?P<name>[a-zA-Z]+)[^0-9-]+(?P<capacity>-?[0-9]+)[^0-9-]+(?P<durability>-?[0-9]+)[^0-9-]+(?P<flavor>-?[0-9]+)[^0-9-]+(?P<texture>-?[0-9]+)[^0-9-]+(?P<calories>-?[0-9]+)").unwrap();
    }
    let mut result: Vec<Ingredient> = Vec::new();
    for line in input_str.trim().lines() {
        match RE_INGREDIENT.captures(line) {
            Some(cap) => {
                let name: String = cap
                    .name("name")
                    .map_or(String::from(""), |x| String::from(x.as_str()));
                let capacity: i64 = cap
                    .name("capacity")
                    .map_or(0i64, |x| x.as_str().parse::<i64>().unwrap());
                let durability: i64 = cap
                    .name("durability")
                    .map_or(0i64, |x| x.as_str().parse::<i64>().unwrap());
                let flavor: i64 = cap
                    .name("flavor")
                    .map_or(0i64, |x| x.as_str().parse::<i64>().unwrap());
                let texture: i64 = cap
                    .name("texture")
                    .map_or(0i64, |x| x.as_str().parse::<i64>().unwrap());
                let calories: i64 = cap
                    .name("calories")
                    .map_or(0i64, |x| x.as_str().parse::<i64>().unwrap());

                result.push(Ingredient::from_data(
                    name, capacity, durability, flavor, texture, calories,
                ));
            }
            None => {
                panic!("Couldn't parse: {}", line);
            }
        }
    }
    result
}

struct Cookie {
    recipe: Vec<(i64,Ingredient)>,
    score: i64,
}

impl Cookie {
    fn from_data(recipe: Vec<(i64,Ingredient)>, score: i64) -> Self {
        Cookie {
            recipe,
            score,
        }
    }
}

fn solve_part_1(input_str: &str) {
    let ingredients = format_input(input_str);
    let cookies: Vec<Cookie> = Vec::new();
    let mut partitions = Partitions::new(100);

    while let Some(partition) = partitions.next() {
        if partition.len() == ingredients.len() {
            for i in 0..ingredients.len() {
                
            }
        }
    }
}

fn main() {
    

}
