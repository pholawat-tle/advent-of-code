use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d1_2015");
    let result = part1(&input);
    println!("{}", result);

    let result = part2(&input);
    println!("{}", result);
}

fn part1(input: &str) -> String {
    let characters = input.chars().into_iter();
    let mut current_floor = 0;

    for character in characters {
        match character {
            '(' => {
                current_floor += 1;
            }
            ')' => {
                current_floor -= 1;
            }
            _ => {}
        }
    }

    current_floor.to_string()
}

fn part2(input: &str) -> String {
    let characters = input.chars().into_iter();
    let mut current_floor = 0;
    let mut index = 0;

    for character in characters {
        match character {
            '(' => {
                current_floor += 1;
                index += 1;
            }
            ')' => {
                current_floor -= 1;
                index += 1;
            }
            _ => {}
        }

        if current_floor == -1 {
            return index.to_string();
        }
    }

    (-1).to_string()
}
