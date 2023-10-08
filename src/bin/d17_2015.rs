use std::collections::HashMap;

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d17_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>()
}

fn part1(input: &str) -> String {
    let mut solution: HashMap<i32, i32> = HashMap::new();
    let containers = parse_input(input);
    find_combination(&containers, 0, 150, 0, &mut solution);

    format!("{}", solution.values().sum::<i32>())
}

fn part2(input: &str) -> String {
    let mut solution: HashMap<i32, i32> = HashMap::new();
    let containers = parse_input(input);
    find_combination(&containers, 0, 150, 0, &mut solution);

    format!("{:?}", solution.keys().min().unwrap())
}

fn find_combination(
    containers: &Vec<i32>,
    offset: usize,
    remaining_eggnog: i32,
    taken: i32,
    solution: &mut HashMap<i32, i32>,
) {
    if remaining_eggnog < 0 {
        return;
    }

    if offset == containers.len() && remaining_eggnog > 0 {
        return;
    }

    if remaining_eggnog == 0 {
        *solution.entry(taken).or_insert(0) += 1;
        return;
    }

    find_combination(
        containers,
        offset + 1,
        remaining_eggnog - containers[offset],
        taken + 1,
        solution,
    );
    find_combination(containers, offset + 1, remaining_eggnog, taken, solution);
}
