#[macro_use]
extern crate lazy_static;
use regex::Regex;
use integer_partitions::Partitions;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone)]
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
    fn from_data(recipe: Vec<(i64,Ingredient)>) -> Self {
        let mut capacity_score: i64 = 0;
        let mut durability_score: i64 = 0;
        let mut flavor_score: i64 = 0;
        let mut texture_score: i64 = 0;
        for (amount, ingredient) in recipe.clone() {
            capacity_score += amount * ingredient.capacity;
            durability_score += amount * ingredient.durability;
            flavor_score += amount * ingredient.flavor;
            texture_score += amount * ingredient.texture;
        }
        let mut score: i64 = 0;
        if capacity_score <= 0 || durability_score <= 0 || flavor_score <= 0 || texture_score <= 0 {
            score = 0;
        } else {
            score = capacity_score * durability_score * flavor_score * texture_score;
        }
        
        Cookie {
            recipe,
            score,
        }
    }
}

fn solve_part_1(input_str: &str) -> i64 {
    let ingredients = format_input(input_str);
    let mut cookies: Vec<Cookie> = Vec::new();
    let mut partitions = Partitions::new(100);

    while let Some(partition) = partitions.next() {
        if partition.len() == ingredients.len() {
            let mut recipe: Vec<(i64, Ingredient)> = Vec::new();
            for i in 0..ingredients.len() {
                recipe.push((partition[i] as i64, ingredients[i].clone()));
            }
            cookies.push(Cookie::from_data(recipe));
        }
    }

    let mut index: usize = 0;
    let mut score: i64 = 0;    
    for (i,cookie) in cookies.iter().enumerate() {
        if cookie.score > score {
            score = cookie.score;
            index = i;
        }
    }    
    score
}

fn main() {
    println!("Answer part 1: {}", solve_part_1(INPUT));

}
