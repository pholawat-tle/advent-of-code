use std::{cmp, collections::HashMap};

use advent_of_code::read_txt;
use itertools::Itertools;

fn main() {
    let input = read_txt("d14_2015");
    let result = part1(&input);
    println!("Result: \n{}", result);
    let result = part2(&input);
    println!("Result: \n{}", result);
}

#[derive(Debug)]
struct Reindeer {
    name: String,
    speed: u32,
    stamina: u32,
    rest_period: u32,
}

fn parse_input(input: &str) -> Vec<Reindeer> {
    let lines = input.lines();

    lines
        .map(|line| {
            let line = line.split(' ').collect_vec();

            Reindeer {
                name: line[0].to_string(),
                speed: line[3].parse().unwrap(),
                stamina: line[6].parse().unwrap(),
                rest_period: line[13].parse().unwrap(),
            }
        })
        .collect()
}

fn calculate_distance(reindeer: &Reindeer, duration: u32) -> u32 {
    let mut remaining_duration = duration as i32;
    let mut total_distance = 0;

    while remaining_duration > 0 {
        let max_duration = cmp::min(remaining_duration as u32, reindeer.stamina);
        let travel_distance = max_duration * reindeer.speed;

        remaining_duration -= max_duration as i32;
        remaining_duration -= reindeer.rest_period as i32;

        total_distance += travel_distance;
    }
    total_distance
}

fn part1(input: &str) -> String {
    let reindeers = parse_input(input);

    reindeers
        .into_iter()
        .map(|reindeer| {
            let distance = calculate_distance(&reindeer, 2503);
            format!("\t{}: {}", reindeer.name, distance)
        })
        .join("\n")
}

fn part2(input: &str) -> String {
    let reindeers = parse_input(input);
    let mut score_map: HashMap<String, u32> = HashMap::new();

    for reindeer in reindeers.iter() {
        score_map.insert(reindeer.name.clone(), 0);
    }

    for time in 1..2504 {
        let distance_with_name = reindeers.iter().map(|reindeer| {
            let distance = calculate_distance(&reindeer, time);
            (reindeer.name.clone(), distance)
        });

        let max_distance = distance_with_name
            .clone()
            .max_by_key(|reindeer| reindeer.1)
            .unwrap()
            .1;

        let leading_reindeers = distance_with_name
            .filter(|reindeer| reindeer.1 == max_distance)
            .map(|reindeer| reindeer.0);

        for reindeer_name in leading_reindeers {
            let score = score_map.get_mut(&reindeer_name).unwrap();
            *score += 1;
        }
    }

    score_map
        .iter()
        .map(|(name, score)| format!("\t{}: {}", name, score))
        .join("\n")
}
