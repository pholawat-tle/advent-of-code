use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d11_2015");
    let result = both_part(&input);
    println!("{}", result);
    let result = both_part(&result);
    println!("{}", result);
}

fn increment(input: &str) -> String {
    let mut result = String::from("");
    let mut chars = input.chars().rev();
    let last_char = chars.next().unwrap();
    let new_char = last_char as u8 + 1;
    let mut carry_over = 0;

    if new_char <= ('z' as u8) {
        result.insert(0, new_char as char);
    } else {
        result.insert(0, 'a');
        carry_over = 1;
    }

    for char in chars {
        if carry_over == 1 {
            let new_char = char as u8 + 1;
            if new_char <= ('z' as u8) {
                result.insert(0, new_char as char);
                carry_over = 0;
            } else {
                result.insert(0, 'a');
                carry_over = 1;
            }
        } else {
            result.insert(0, char);
        }
    }

    if carry_over == 1 {
        result.insert(0, 'a');
    }

    result
}

fn both_part(input: &str) -> String {
    let mut input = increment(input);

    while !match_condition(&input) {
        input = increment(&input);
    }

    input
}

fn match_condition(input: &str) -> bool {
    let mut prev_char: Option<char> = None;
    let mut prev_prev_char: Option<char> = None;

    let mut has_straight = false;

    let mut pair_found = 0;

    for char in input.chars() {
        if char == 'i' || char == 'o' || char == 'l' {
            return false;
        }

        if prev_char.is_some() && prev_prev_char.is_none() {
            if char == prev_char.unwrap() {
                pair_found += 1;
            }
        }

        if prev_char.is_some() && prev_prev_char.is_some() {
            let prev_char = prev_char.unwrap();
            let prev_prev_char = prev_prev_char.unwrap();
            if char as u8 == prev_char as u8 + 1 && char as u8 == prev_prev_char as u8 + 2 {
                has_straight = true;
            }

            if char == prev_char && char != prev_prev_char {
                pair_found += 1;
            }
        }

        prev_prev_char = prev_char;
        prev_char = Some(char);
    }

    has_straight && pair_found >= 2
}
