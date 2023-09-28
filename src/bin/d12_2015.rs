use advent_of_code::read_txt;
use regex::Regex;

fn main() {
    let input = read_txt("d12_2015");
    let result = part1(&input);
    println!("{}", result);
}

fn part1(input: &str) -> String {
    let number_regex = Regex::new(r"-?[\d]+").expect("Invalid regex");
    let numbers_iter = number_regex.captures_iter(&input);

    let sum: i32 = numbers_iter
        .filter_map(|number_block| {
            let number = match number_block.get(0) {
                Some(number) => number.as_str(),
                None => return None,
            };

            let number: i32 = match number.parse() {
                Ok(number) => number,
                Err(_) => return None,
            };

            Some(number)
        })
        .sum();

    sum.to_string()
}
