use std::collections::{HashMap, HashSet};

use advent_of_code::read_txt;
use itertools::Itertools;

fn main() {
    let input = read_txt("d19_2015");
    let result = part1(&input);
    println!("{}", result);
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<&str>>, &str) {
    let mut input = input.split("\n\n").into_iter();

    let substitutions = input
        .next()
        .unwrap()
        .lines()
        .map(|line| line.trim().split(" => ").collect_vec())
        .fold(HashMap::new(), |substitions_map, substition| {
            let mut substitions_map = substitions_map;
            let (key, value) = (substition[0], substition[1]);
            let key_entry = substitions_map.entry(key).or_insert_with(|| vec![]);
            key_entry.push(value);

            substitions_map
        });
    let original_text = input.next().unwrap().trim();

    (substitutions, original_text)
}

fn part1(input: &str) -> String {
    let (substitutions, original_text) = parse_input(input);
    let mut original_text_iter = original_text.chars().enumerate().peekable();
    let mut seen: HashSet<String> = HashSet::new();

    while let Some((index, character)) = original_text_iter.next() {
        let is_single_character = match original_text_iter.peek() {
            Some((_, next_character)) => next_character.is_ascii_uppercase(),
            None => true,
        };

        let molecule = if is_single_character {
            character.to_string()
        } else {
            format!("{}{}", character, original_text_iter.next().unwrap().1)
        };

        if let Some(substitutions) = substitutions.get(&molecule[..]) {
            for substitution in substitutions {
                let mut new_text = original_text.to_string();
                new_text.replace_range(index..(index + molecule.len()), substitution);
                seen.insert(new_text);
            }
        }
    }

    seen.len().to_string()
}
