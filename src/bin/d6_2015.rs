use std::{collections::HashMap, str::Lines};

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d6_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

type Coordinate = (u32, u32);

enum InstructionType {
    Toggle,
    TurnOn,
    TurnOff,
}

trait Grid {
    fn execute_instruction(&mut self, i_type: InstructionType, c1: Coordinate, c2: Coordinate);

    fn bulk_execute_command(&mut self, lines: Lines) {
        for line in lines {
            let words = line.split_whitespace().collect::<Vec<&str>>();

            if words[0] == "toggle" {
                let c1 = words[1]
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let c1 = (c1[0], c1[1]);

                let c2 = words[3]
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let c2 = (c2[0], c2[1]);

                self.execute_instruction(InstructionType::Toggle, c1, c2);
            } else {
                let c1 = words[2]
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let c1 = (c1[0], c1[1]);

                let c2 = words[4]
                    .split(',')
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let c2 = (c2[0], c2[1]);

                if words[1] == "on" {
                    self.execute_instruction(InstructionType::TurnOn, c1, c2);
                } else if words[1] == "off" {
                    self.execute_instruction(InstructionType::TurnOff, c1, c2);
                }
            }
        }
    }
}

struct GridV1 {
    map: HashMap<Coordinate, bool>,
}

impl GridV1 {
    fn new() -> GridV1 {
        let mut map: HashMap<Coordinate, bool> = HashMap::new();

        for i in 0..1000 {
            for j in 0..1000 {
                map.insert((i, j), false);
            }
        }

        GridV1 { map }
    }
}

impl Grid for GridV1 {
    fn execute_instruction(&mut self, i_type: InstructionType, c1: Coordinate, c2: Coordinate) {
        let bottom_left = (c1.0.min(c2.0), c1.1.min(c2.1));
        let top_right = (c1.0.max(c2.0), c1.1.max(c2.1));

        for i in bottom_left.0..(top_right.0 + 1) {
            for j in bottom_left.1..(top_right.1 + 1) {
                match i_type {
                    InstructionType::TurnOn => {
                        self.map.insert((i, j), true);
                    }
                    InstructionType::TurnOff => {
                        self.map.insert((i, j), false);
                    }
                    InstructionType::Toggle => {
                        let previous_value = self.map.get(&(i, j)).unwrap();
                        self.map.insert((i, j), !previous_value);
                    }
                }
            }
        }
    }
}

fn part1(input: &str) -> String {
    let mut grid = GridV1::new();
    grid.bulk_execute_command(input.lines());

    let mut lit_count = 0;
    for i in grid.map.values() {
        if *i {
            lit_count += 1;
        }
    }

    lit_count.to_string()
}

struct GridV2 {
    map: HashMap<Coordinate, u32>,
}

impl GridV2 {
    fn new() -> GridV2 {
        let mut map: HashMap<Coordinate, u32> = HashMap::new();

        for i in 0..1000 {
            for j in 0..1000 {
                map.insert((i, j), 0);
            }
        }

        GridV2 { map }
    }
}

impl Grid for GridV2 {
    fn execute_instruction(&mut self, i_type: InstructionType, c1: Coordinate, c2: Coordinate) {
        let bottom_left = (c1.0.min(c2.0), c1.1.min(c2.1));
        let top_right = (c1.0.max(c2.0), c1.1.max(c2.1));

        for i in bottom_left.0..(top_right.0 + 1) {
            for j in bottom_left.1..(top_right.1 + 1) {
                let previous_value = self.map.get(&(i, j)).unwrap();
                match i_type {
                    InstructionType::TurnOn => {
                        self.map.insert((i, j), previous_value + 1);
                    }
                    InstructionType::TurnOff => {
                        if *previous_value > 0 {
                            self.map.insert((i, j), previous_value - 1);
                        }
                    }
                    InstructionType::Toggle => {
                        self.map.insert((i, j), previous_value + 2);
                    }
                }
            }
        }
    }
}

fn part2(input: &str) -> String {
    let mut grid = GridV2::new();
    grid.bulk_execute_command(input.lines());

    let mut total_brightness = 0;
    for i in grid.map.values() {
        total_brightness += *i;
    }

    total_brightness.to_string()
}
