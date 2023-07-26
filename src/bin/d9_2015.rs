use std::collections::{HashMap, HashSet};

use advent_of_code::read_txt;
use itertools::Itertools;

fn main() {
    let input = read_txt("d9_2015");
    let result = part_1(&input);
    println!("{}", result);
    let result = part_2(&input);
    println!("{}", result);
}

fn parse_destination_and_distance<'a>(
    iter: impl Iterator<Item = &'a str>,
) -> (HashSet<&'a str>, HashMap<(&'a str, &'a str), usize>) {
    iter.fold((HashSet::new(), HashMap::new()), |mut prev_result, line| {
        let line: Vec<&str> = line.split_whitespace().collect();
        prev_result.0.insert(line[0]);
        prev_result.0.insert(line[2]);

        prev_result
            .1
            .insert((line[0], line[2]), line[4].parse().unwrap());

        prev_result
    })
}

fn calculate_total_distance(
    route: Vec<&str>,
    distance_map: &HashMap<(&str, &str), usize>,
) -> Option<usize> {
    let mut total_distance = 0;
    for i in 0..route.len() - 1 {
        total_distance += match (
            distance_map.get(&(route[i], route[i + 1])),
            distance_map.get(&(route[i + 1], route[i])),
        ) {
            (Some(distance), _) => *distance,
            (_, Some(distance)) => *distance,
            _ => return None,
        }
    }
    Some(total_distance)
}

fn part_1(input: &str) -> String {
    let (destinations, distance_map) = parse_destination_and_distance(input.lines());
    let number_of_destination = destinations.len();
    let permutations = destinations
        .clone()
        .into_iter()
        .permutations(number_of_destination);

    permutations
        .fold(None, |current_minimum: Option<usize>, route| {
            let total_distance = calculate_total_distance(route, &distance_map);

            let current_minimum = match current_minimum {
                Some(current_minimum) => current_minimum,
                None => return total_distance,
            };

            let total_distance = match total_distance {
                Some(total_distance) => total_distance,
                None => return Some(current_minimum),
            };

            Some(current_minimum.min(total_distance))
        })
        .unwrap()
        .to_string()
}

fn part_2(input: &str) -> String {
    let (destinations, distance_map) = parse_destination_and_distance(input.lines());
    let number_of_destination = destinations.len();
    let permutations = destinations
        .clone()
        .into_iter()
        .permutations(number_of_destination);

    permutations
        .fold(None, |current_minimum: Option<usize>, route| {
            let total_distance = calculate_total_distance(route, &distance_map);

            let current_minimum = match current_minimum {
                Some(current_minimum) => current_minimum,
                None => return total_distance,
            };

            let total_distance = match total_distance {
                Some(total_distance) => total_distance,
                None => return Some(current_minimum),
            };

            Some(current_minimum.max(total_distance))
        })
        .unwrap()
        .to_string()
}
