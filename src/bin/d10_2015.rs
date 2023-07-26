use advent_of_code::read_txt;
use itertools::Itertools;

fn main() {
    let input = read_txt("d10_2015");
    let result = part_1(&input);
    println!("{}", result);
    let result = part_2(&input);
    println!("{}", result);
}

fn part_1(input: &str) -> String {
    let mut input = input.to_string();

    for _ in 0..40 {
        let chars = input.trim().chars();

        let mut result: Vec<(u32, u32)> = vec![];
        let mut prev_char: Option<char> = None;

        for char in chars {
            match prev_char {
                Some(prev_char) => {
                    if prev_char == char {
                        result.last_mut().unwrap().0 += 1;
                    } else {
                        result.push((1, char as u32 - '0' as u32));
                    }
                }
                None => {
                    result.push((1, char as u32 - '0' as u32));
                }
            }
            prev_char = Some(char);
        }

        input = result
            .into_iter()
            .map(|i| vec![i.0, i.1])
            .flatten()
            .join("");
    }

    input.len().to_string()
}

fn part_2(input: &str) -> String {
    let mut input = input.to_string();

    for _ in 0..50 {
        let chars = input.trim().chars();

        let mut result: Vec<(u32, u32)> = vec![];
        let mut prev_char: Option<char> = None;

        for char in chars {
            match prev_char {
                Some(prev_char) => {
                    if prev_char == char {
                        result.last_mut().unwrap().0 += 1;
                    } else {
                        result.push((1, char as u32 - '0' as u32));
                    }
                }
                None => {
                    result.push((1, char as u32 - '0' as u32));
                }
            }
            prev_char = Some(char);
        }

        input = result
            .into_iter()
            .map(|i| vec![i.0, i.1])
            .flatten()
            .join("");
    }

    input.len().to_string()
}
