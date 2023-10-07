use std::collections::HashMap;

use advent_of_code::read_txt;
use itertools::Itertools;
use regex::Regex;

const MFCSAM_OUTPUT: &str = 
    r"children: 3 cats: 7 samoyeds: 2 pomeranians: 3 akitas: 0 vizslas: 0 goldfish: 5 trees: 3 cars: 2 perfumes: 1";

fn main() {
    let input = read_txt("d16_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

fn part1(input: &str) -> String {
    let regex = Regex::new(r"([\w]+):\s([\d]+)").expect("Invalid regex");
    let solution_items: HashMap<String, u32> = regex
        .captures_iter(MFCSAM_OUTPUT)
        .map(|cap| {
            let key = cap[1].to_string();
            let value = cap[2].parse::<u32>().unwrap();
            (key, value)
        })
        .collect();

    let aunts_items = input.lines().map(|line| {
        regex
            .captures_iter(line)
            .map(|cap| {
                let key = cap[1].to_string();
                let value = cap[2].parse::<u32>().unwrap();
                (key, value)
            })
            .collect_vec()
    });

    'outer: for (i, aunt_items) in aunts_items.enumerate() {
        for item in aunt_items {
            let key = item.0;
            let value = item.1;

            if solution_items.contains_key(&key) {
                if solution_items.get(&key).unwrap() != &value {
                    continue 'outer;
                }
            }
        }

        return (i + 1).to_string();
    }

    unreachable!()
}

fn part2(input: &str) -> String {
    let regex = Regex::new(r"([\w]+):\s([\d]+)").expect("Invalid regex");
    let solution_items: HashMap<String, u32> = regex
        .captures_iter(MFCSAM_OUTPUT)
        .map(|cap| {
            let key = cap[1].to_string();
            let value = cap[2].parse::<u32>().unwrap();
            (key, value)
        })
        .collect();

    let aunts_items = input.lines().map(|line| {
        regex
            .captures_iter(line)
            .map(|cap| {
                let key = cap[1].to_string();
                let value = cap[2].parse::<u32>().unwrap();
                (key, value)
            })
            .collect_vec()
    });

    'outer: for (i, aunt_items) in aunts_items.enumerate() {
        for item in aunt_items {
            let key = item.0;
            let value = item.1;

            if solution_items.contains_key(&key) {
                if key == "cats" || key == "trees" {
                    if solution_items.get(&key).unwrap() >= &value {
                        continue 'outer;
                    }
                } else if key == "pomeranians" || key == "goldfish" {
                    if solution_items.get(&key).unwrap() <= &value {
                        continue 'outer;
                    }
                } else {
                    if solution_items.get(&key).unwrap() != &value {
                        continue 'outer;
                    }
                }
            }
        }

        return (i + 1).to_string();
    }

    unreachable!()
}
