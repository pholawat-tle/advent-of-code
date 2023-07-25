use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d8_2015");
    let result = part_1(&input);
    println!("{}", result);
    let result = part_2(&input);
    println!("{}", result);
}

fn part_1(input: &str) -> String {
    let result = input.lines().fold((0, 0), |acc, line| {
        let mut chars = line.trim().chars().peekable();
        let mut code_count = 0;
        let mut char_count = 0;

        while let Some(next_char) = chars.next() {
            match next_char {
                '\\' => match chars.peek() {
                    Some('x') => {
                        chars.next();
                        chars.next();
                        chars.next();
                        code_count += 4;
                        char_count += 1;
                    }
                    Some('\"') => {
                        chars.next();
                        code_count += 2;
                        char_count += 1;
                    }
                    Some('\\') => {
                        chars.next();
                        code_count += 2;
                        char_count += 1;
                    }
                    _ => {}
                },
                _ => {
                    code_count += 1;
                    char_count += 1;
                }
            }
        }

        (
            code_count + acc.0,
            char_count - 2 + acc.1, // Subtracting 2 out of char_count for the quotes
        )
    });

    format!("{}", result.0 - result.1)
}

fn part_2(input: &str) -> String {
    let result = input.lines().fold((0, 0), |acc, line| {
        let line = line.trim();
        let original_count: u64 = line.len().try_into().unwrap();
        let mut new_count = 0;

        let chars = line.chars();

        for char in chars {
            match char {
                '"' => new_count += 2,
                '\\' => new_count += 2,
                _ => new_count += 1,
            }
        }

        (
            original_count + acc.0,
            new_count + acc.1 + 2, // Adding 2 for the quotes
        )
    });

    format!("{}", result.1 - result.0)
}
