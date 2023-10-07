use std::{cmp, collections::HashMap};

use advent_of_code::read_txt;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = read_txt("d15_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_input(input: &str) -> HashMap<String, Ingredient> {
    let lines = input.lines();
    let number_regex = Regex::new(r"-?[\d]+").expect("Invalid regex");

    lines.fold(HashMap::new(), |mut ingredients_map, line| {
        let name = line.split(":").next().expect("Invalid line");
        let numbers = number_regex
            .captures_iter(line)
            .map(|number| {
                number[0]
                    .parse::<i32>()
                    .expect("Impossible! Invalid number")
            })
            .collect_vec();

        let ingredient = Ingredient {
            capacity: numbers[0],
            durability: numbers[1],
            flavor: numbers[2],
            texture: numbers[3],
            calories: numbers[4],
        };

        ingredients_map.insert(name.to_string(), ingredient);

        ingredients_map
    })
}

fn let_the_man_cook(
    frosting: i32,
    candy: i32,
    butterscotch: i32,
    sugar: i32,
    ingredients_map: &HashMap<String, Ingredient>,
) -> (i32, i32) {
    let capacity = ingredients_map.get("Sugar").unwrap().capacity * sugar
        + ingredients_map.get("Butterscotch").unwrap().capacity * butterscotch
        + ingredients_map.get("Candy").unwrap().capacity * candy
        + ingredients_map.get("Frosting").unwrap().capacity * frosting;
    let capacity = capacity * if capacity > 0 { 1 } else { 0 };

    let durability = ingredients_map.get("Sugar").unwrap().durability * sugar
        + ingredients_map.get("Butterscotch").unwrap().durability * butterscotch
        + ingredients_map.get("Candy").unwrap().durability * candy
        + ingredients_map.get("Frosting").unwrap().durability * frosting;
    let durability = durability * if durability > 0 { 1 } else { 0 };

    let flavor = ingredients_map.get("Sugar").unwrap().flavor * sugar
        + ingredients_map.get("Butterscotch").unwrap().flavor * butterscotch
        + ingredients_map.get("Candy").unwrap().flavor * candy
        + ingredients_map.get("Frosting").unwrap().flavor * frosting;
    let flavor = flavor * if flavor > 0 { 1 } else { 0 };

    let texture = ingredients_map.get("Sugar").unwrap().texture * sugar
        + ingredients_map.get("Butterscotch").unwrap().texture * butterscotch
        + ingredients_map.get("Candy").unwrap().texture * candy
        + ingredients_map.get("Frosting").unwrap().texture * frosting;
    let texture = texture * if texture > 0 { 1 } else { 0 };

    let calories = ingredients_map.get("Sugar").unwrap().calories * sugar
        + ingredients_map.get("Butterscotch").unwrap().calories * butterscotch
        + ingredients_map.get("Candy").unwrap().calories * candy
        + ingredients_map.get("Frosting").unwrap().calories * frosting;

    (capacity * durability * flavor * texture, calories)
}

fn part1(input: &str) -> String {
    let ingredients_map = parse_input(input);
    let mut max_score = 0;

    for frosting in 0..101 {
        for candy in 0..(101 - frosting) {
            for butterscotch in 0..(101 - frosting - candy) {
                let sugar = 100 - frosting - candy - butterscotch;
                let (result, _) =
                    let_the_man_cook(frosting, candy, butterscotch, sugar, &ingredients_map);

                max_score = cmp::max(result, max_score);
            }
        }
    }

    max_score.to_string()
}

fn part2(input: &str) -> String {
    let ingredients_map = parse_input(input);
    let mut max_score = 0;

    for frosting in 0..101 {
        for candy in 0..(101 - frosting) {
            for butterscotch in 0..(101 - frosting - candy) {
                let sugar = 100 - frosting - candy - butterscotch;
                let (result, calories) =
                    let_the_man_cook(frosting, candy, butterscotch, sugar, &ingredients_map);

                max_score = if calories == 500 {
                    cmp::max(result, max_score)
                } else {
                    max_score
                }
            }
        }
    }

    max_score.to_string()
}
