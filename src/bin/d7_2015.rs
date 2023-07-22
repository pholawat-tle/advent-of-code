use std::collections::HashMap;

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d7_2015");
    let result = part1(&input);
    println!("{}", result);
    // For part 2, replace the value of wire b in the input file with the result of part 1
}

#[derive(Debug)]
struct Register {
    map: HashMap<String, u16>,
}

impl Register {
    fn new() -> Register {
        Register {
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, register: &str, value: u16) {
        self.map.insert(register.to_string(), value);
    }

    fn eval(&self, instruction: &str) -> Result<u16, &str> {
        let instruction: Vec<&str> = instruction.split_whitespace().collect();

        match instruction.len() {
            1 => {
                let is_number = instruction[0].parse::<u16>().is_ok();

                let value: u16 = if is_number {
                    instruction[0].parse().unwrap()
                } else {
                    let register_value = self.map.get(instruction[0]);

                    let register_value = if register_value.is_none() {
                        return Err("key not exist yet, skipping");
                    } else {
                        *register_value.unwrap()
                    };

                    register_value
                };

                Ok(value)
            }
            2 => {
                let is_number = instruction[1].parse::<u16>().is_ok();

                let value: u16 = if is_number {
                    instruction[1].parse().unwrap()
                } else {
                    let register_value = self.map.get(instruction[1]);

                    let register_value = if register_value.is_none() {
                        return Err("key not exist yet, skipping");
                    } else {
                        *register_value.unwrap()
                    };

                    register_value
                };

                Ok(!value)
            }
            3 => {
                let is_left_number = instruction[0].parse::<u16>().is_ok();
                let is_right_number = instruction[2].parse::<u16>().is_ok();

                let l_value: u16 = if is_left_number {
                    instruction[0].parse().unwrap()
                } else {
                    let register_value = self.map.get(instruction[0]);

                    let register_value = if register_value.is_none() {
                        return Err("key not exist yet, skipping");
                    } else {
                        *register_value.unwrap()
                    };

                    register_value
                };
                let r_value: u16 = if is_right_number {
                    instruction[2].parse().unwrap()
                } else {
                    let register_value = self.map.get(instruction[2]);

                    let register_value = if register_value.is_none() {
                        return Err("key not exist yet, skipping");
                    } else {
                        *register_value.unwrap()
                    };

                    register_value
                };

                let value = if instruction[1] == "AND" {
                    l_value & r_value
                } else if instruction[1] == "OR" {
                    l_value | r_value
                } else if instruction[1] == "LSHIFT" {
                    l_value << r_value
                } else if instruction[1] == "RSHIFT" {
                    l_value >> r_value
                } else {
                    0
                };

                Ok(value)
            }
            _ => Err("instruction type not found"),
        }
    }
}

type Instruction = (String, String);

fn part1(input: &str) -> String {
    let mut register = Register::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in input.lines() {
        let line: Vec<&str> = line.split("->").collect();

        let lhs = line[0].trim();
        let rhs = line[1].trim();

        instructions.push((lhs.to_string(), rhs.to_string()));
    }

    while instructions.len() > 0 {
        let (lhs, rhs) = instructions.remove(0);

        let result = register.eval(&lhs);

        if result.is_err() {
            instructions.push((lhs, rhs));
            continue;
        }

        let result = result.unwrap();
        register.insert(&rhs, result);
    }

    format!("{:#?}", register.map.get("a"))
}
