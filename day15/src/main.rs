#[macro_use]
extern crate lazy_static;
use regex::Regex;
const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn from_data(
        name: String,
        capacity: i32,
        durability: i32,
        flavor: i32,
        texture: i32,
        calories: i32,
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
                let capacity: i32 = cap
                    .name("capacity")
                    .map_or(0i32, |x| x.as_str().parse::<i32>().unwrap());
                let durability: i32 = cap
                    .name("durability")
                    .map_or(0i32, |x| x.as_str().parse::<i32>().unwrap());
                let flavor: i32 = cap
                    .name("flavor")
                    .map_or(0i32, |x| x.as_str().parse::<i32>().unwrap());
                let texture: i32 = cap
                    .name("texture")
                    .map_or(0i32, |x| x.as_str().parse::<i32>().unwrap());
                let calories: i32 = cap
                    .name("calories")
                    .map_or(0i32, |x| x.as_str().parse::<i32>().unwrap());

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

fn main() {
    let ingredients = format_input(INPUT);
    println!("{:?}", ingredients);
}
