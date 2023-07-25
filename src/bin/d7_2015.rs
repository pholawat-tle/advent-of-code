use std::collections::HashMap;

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d7_2015");
    let result = both_part(&input);
    println!("{}", result);
}

#[derive(Debug, Clone)]
enum Instruction<'a> {
    NOT(&'a str),
    AND(&'a str, &'a str),
    OR(&'a str, &'a str),
    LSHIFT(&'a str, u16),
    RSHIFT(&'a str, u16),
    ASSIGN(&'a str),
    NUMBER(u16),
}

fn both_part(input: &str) -> String {
    let mut instructions: HashMap<&str, Instruction> = HashMap::new();

    for line in input.lines() {
        let line: Vec<&str> = line.split("->").collect();

        let key = line[1].trim();
        let instruction = line[0].trim();

        let instruction: Vec<&str> = instruction.split_whitespace().collect();

        match instruction.len() {
            1 => {
                let value = instruction[0];
                instructions.insert(key, Instruction::ASSIGN(value));
            }
            2 => {
                let value = instruction[1];
                instructions.insert(key, Instruction::NOT(value));
            }
            3 => {
                let left_value = instruction[0];
                let right_value = instruction[2];

                match instruction[1] {
                    "AND" => instructions.insert(key, Instruction::AND(left_value, right_value)),
                    "OR" => instructions.insert(key, Instruction::OR(left_value, right_value)),
                    "LSHIFT" => instructions.insert(
                        key,
                        Instruction::LSHIFT(left_value, right_value.parse::<u16>().unwrap()),
                    ),
                    "RSHIFT" => instructions.insert(
                        key,
                        Instruction::RSHIFT(left_value, right_value.parse::<u16>().unwrap()),
                    ),
                    _ => panic!("Unknown instruction"),
                };
            }
            _ => panic!("Unknown instruction"),
        }
    }

    format!("{:#?}", calculate("a", &mut instructions))
}

fn calculate<'a>(character: &'a str, instructions: &mut HashMap<&'a str, Instruction<'a>>) -> u16 {
    if let Ok(number) = character.parse::<u16>() {
        return number;
    }

    let solution = instructions.remove(character).unwrap();
    let result = match solution {
        Instruction::NUMBER(number) => number,
        Instruction::ASSIGN(value) => {
            let result = calculate(value, instructions);
            result
        }
        Instruction::NOT(value) => {
            let result = !calculate(value, instructions);
            result
        }
        Instruction::AND(left_value, right_value) => {
            let result = calculate(left_value, instructions) & calculate(right_value, instructions);
            result
        }
        Instruction::OR(left_value, right_value) => {
            let result = calculate(left_value, instructions) | calculate(right_value, instructions);
            result
        }
        Instruction::LSHIFT(left_value, right_value) => {
            let result = calculate(left_value, instructions) << right_value;
            result
        }
        Instruction::RSHIFT(left_value, right_value) => {
            let result = calculate(left_value, instructions) >> right_value;
            result
        }
    };

    instructions.insert(character, Instruction::NUMBER(result));
    result
}
