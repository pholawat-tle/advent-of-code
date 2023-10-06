use std::collections::HashSet;

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d1_2016");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug)]
enum TurnDirection {
    Left,
    Right,
}

fn get_distance(step: i32, direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::North => (0, step),
        Direction::South => (0, -step),
        Direction::East => (step, 0),
        Direction::West => (-step, 0),
    }
}

fn get_new_direction(current_direction: &Direction, turn_direction: &TurnDirection) -> Direction {
    match current_direction {
        Direction::North => match turn_direction {
            TurnDirection::Left => Direction::West,
            TurnDirection::Right => Direction::East,
        },
        Direction::South => match turn_direction {
            TurnDirection::Left => Direction::East,
            TurnDirection::Right => Direction::West,
        },
        Direction::East => match turn_direction {
            TurnDirection::Left => Direction::North,
            TurnDirection::Right => Direction::South,
        },
        Direction::West => match turn_direction {
            TurnDirection::Left => Direction::South,
            TurnDirection::Right => Direction::North,
        },
    }
}

fn part1(input: &str) -> String {
    let instructions = input.split(",").map(|instruction| instruction.trim());

    let mut current_coordinate = (0, 0);
    let mut current_direction = Direction::North;

    instructions.for_each(|instruction| {
        let (turn_direction, step) = parse_instruction_str(instruction);

        let new_direction = get_new_direction(&current_direction, &turn_direction);
        let distance = get_distance(step, &new_direction);

        current_coordinate = (
            current_coordinate.0 + distance.0,
            current_coordinate.1 + distance.1,
        );
        current_direction = new_direction;
    });

    format!("{:?}", current_coordinate)
}

fn part2(input: &str) -> String {
    let instructions = input.split(",").map(|instruction| instruction.trim());

    let mut current_coordinate = (0, 0);
    let mut current_direction = Direction::North;
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(current_coordinate);

    for instruction in instructions {
        let (turn_direction, step) = parse_instruction_str(instruction);

        let new_direction = get_new_direction(&current_direction, &turn_direction);

        for _ in 0..step {
            let distance = get_distance(1, &new_direction);

            current_coordinate = (
                current_coordinate.0 + distance.0,
                current_coordinate.1 + distance.1,
            );

            if visited.contains(&current_coordinate) {
                return format!("{:?}", current_coordinate);
            }

            visited.insert(current_coordinate);
        }

        current_direction = new_direction;
    }

    panic!("No location visited twice")
}

fn parse_instruction_str(instruction: &str) -> (TurnDirection, i32) {
    let (turn_direction, step) = instruction.split_at(1);
    let turn_direction = match turn_direction {
        "L" => TurnDirection::Left,
        "R" => TurnDirection::Right,
        _ => panic!("Invalid turn direction"),
    };
    let step = step.parse::<i32>().unwrap();

    (turn_direction, step)
}
