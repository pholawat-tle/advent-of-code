use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use advent_of_code::read_txt;
use itertools::Itertools;

fn main() {
    let input = read_txt("d13_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

fn parse_input(input: &str) -> (HashSet<String>, HashMap<(String, String), i32>) {
    let mut unique_people: HashSet<String> = HashSet::new();
    let mut happiness_map: HashMap<(String, String), i32> = HashMap::new();
    let lines = input.lines();

    let lines = lines
        .map(|line| {
            let line = line.split(' ');
            line.collect_vec()
        })
        .collect_vec();

    for line in lines {
        let person1 = line[0].to_string();
        let person2 = line[10].trim_end_matches('.').to_string();
        let is_positive = line[2] == "gain";
        let score = line[3].parse::<i32>().unwrap();

        unique_people.insert(person1.to_string());
        unique_people.insert(person2.to_string());

        happiness_map.insert(
            (person1, person2),
            match is_positive {
                true => score,
                false => -score,
            },
        );
    }

    (unique_people, happiness_map)
}

fn calculate_score(
    number_of_people: usize,
    happiness_map: &HashMap<(String, String), i32>,
    layout: Vec<String>,
) -> i32 {
    let mut total_change_of_happiness = 0;
    for i in 0..number_of_people {
        let current_person = &layout[i];
        let person_to_the_left = &layout[(i + number_of_people - 1) % number_of_people];
        let person_to_the_right = &layout[(i + number_of_people + 1) % number_of_people];

        let left_score =
            happiness_map.get(&(current_person.to_string(), person_to_the_left.to_string()));
        let right_score =
            happiness_map.get(&(current_person.to_string(), person_to_the_right.to_string()));

        total_change_of_happiness += match left_score {
            Some(score) => *score,
            None => 0,
        };

        total_change_of_happiness += match right_score {
            Some(score) => *score,
            None => 0,
        };
    }

    total_change_of_happiness
}

fn part1(input: &str) -> String {
    let (unique_people, happiness_map) = parse_input(input);

    let number_of_people = unique_people.len();
    let permutations = unique_people
        .clone()
        .into_iter()
        .permutations(number_of_people);

    let result = permutations.fold(None, |current_maximum: Option<i32>, layout| {
        let total_change_of_happiness = calculate_score(number_of_people, &happiness_map, layout);

        match current_maximum {
            Some(maximum) => Some(cmp::max(maximum, total_change_of_happiness)),
            None => Some(total_change_of_happiness),
        }
    });

    format!("{:#?}", result)
}

fn part2(input: &str) -> String {
    let (mut unique_people, mut happiness_map) = parse_input(input);

    for person in &unique_people {
        happiness_map.insert(("Me".to_string(), person.to_string()), 0);
        happiness_map.insert((person.to_string(), "Me".to_string()), 0);
    }

    unique_people.insert("Me".to_string());

    let number_of_people = unique_people.len();
    let permutations = unique_people
        .clone()
        .into_iter()
        .permutations(number_of_people);

    let result = permutations.fold(None, |current_maximum: Option<i32>, layout| {
        let total_change_of_happiness = calculate_score(number_of_people, &happiness_map, layout);

        match current_maximum {
            Some(maximum) => Some(cmp::max(maximum, total_change_of_happiness)),
            None => Some(total_change_of_happiness),
        }
    });

    format!("{:#?}", result)
}
