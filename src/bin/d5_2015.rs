use std::collections::HashSet;

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d5_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

fn part1(input: &str) -> String {
    let mut nice_strings = 0;

    for line in input.lines() {
        if is_nice(line) {
            nice_strings += 1;
        }
    }

    nice_strings.to_string()
}

fn is_nice(input: &str) -> bool {
    let mut prev_char: Option<char> = None;

    let mut vowel_count = 0;
    let mut has_double = false;
    let mut contain_bad_string = false;

    for char in input.chars() {
        if char == 'a' || char == 'e' || char == 'i' || char == 'o' || char == 'u' {
            vowel_count += 1;
        }

        if let Some(prev_char) = prev_char {
            if prev_char == char {
                has_double = true;
            }

            if prev_char == 'a' && char == 'b' {
                contain_bad_string = true;
            } else if prev_char == 'c' && char == 'd' {
                contain_bad_string = true;
            } else if prev_char == 'p' && char == 'q' {
                contain_bad_string = true;
            } else if prev_char == 'x' && char == 'y' {
                contain_bad_string = true;
            }
        }

        prev_char = Some(char);
    }

    vowel_count >= 3 && has_double && !contain_bad_string
}

fn part2(input: &str) -> String {
    let mut nice_strings = 0;

    for line in input.lines() {
        if is_nice2(line) {
            nice_strings += 1;
        }
    }

    nice_strings.to_string()
}

fn is_nice2(input: &str) -> bool {
    let mut pair_set: HashSet<(char, char)> = HashSet::new();

    let mut last_char: Option<char> = None;
    let mut second_last_char: Option<char> = None;
    let mut last_pair: Option<(char, char)> = None;

    let mut has_pair_twice = false;
    let mut has_pair_with_letter_in_between = false;

    for char in input.chars() {
        if let Some(prev_char) = last_char {
            let pair = (prev_char, char);
            let is_new_pair = !pair_set.insert(pair);
            let not_overlapped_with_last_pair =
                last_pair.is_none() || (last_pair.is_some() && pair != last_pair.unwrap());

            if is_new_pair && not_overlapped_with_last_pair {
                has_pair_twice = true;
            }

            last_pair = Some(pair)
        }

        if let Some(prev_prev_char) = second_last_char {
            if prev_prev_char == char && prev_prev_char != last_char.unwrap() {
                has_pair_with_letter_in_between = true;
            }
        }

        second_last_char = last_char;
        last_char = Some(char);
    }

    has_pair_twice && has_pair_with_letter_in_between
}
