use std::collections::HashMap;

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d3_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

type Coordinate = (i32, i32);

fn part1(input: &str) -> String {
    let mut map: HashMap<Coordinate, u32> = HashMap::new();
    let orders = input.chars().into_iter();
    let mut current_coordinate = (0, 0);

    map.insert(current_coordinate, 1);
    let mut at_least_once = 1;

    for order in orders {
        match order {
            '<' => {
                current_coordinate.0 -= 1;
            }
            '>' => {
                current_coordinate.0 += 1;
            }
            '^' => {
                current_coordinate.1 += 1;
            }
            'v' => {
                current_coordinate.1 -= 1;
            }
            _ => {}
        }

        let next_coordinate_visits = map.get(&current_coordinate);
        if next_coordinate_visits.is_none() {
            at_least_once += 1;
            map.insert(current_coordinate, 1);
        }
    }

    at_least_once.to_string()
}

struct CoordinateSwitcher<'a> {
    coordinates: Vec<&'a mut Coordinate>,
}

impl<'a> CoordinateSwitcher<'a> {
    fn get_coordinate(&self, turn: usize) -> Coordinate {
        **(self.coordinates.get(turn % self.coordinates.len()).unwrap())
    }

    fn mutate_coordinate(&mut self, turn: usize, mutation: Coordinate) {
        let size = self.coordinates.len();
        let coordinate = self.coordinates.get_mut(turn % size).unwrap();

        coordinate.0 += mutation.0;
        coordinate.1 += mutation.1;
    }
}

fn part2(input: &str) -> String {
    let mut map: HashMap<Coordinate, u32> = HashMap::new();
    let orders = input.chars().into_iter();

    let mut santa_coordinate = (0, 0);
    let mut robo_coordinate = (0, 0);
    let mut turn = 0;

    let mut switcher = CoordinateSwitcher {
        coordinates: vec![&mut santa_coordinate, &mut robo_coordinate],
    };

    map.insert(switcher.get_coordinate(turn), 1);
    let mut at_least_once = 1;

    for order in orders {
        match order {
            '<' => {
                switcher.mutate_coordinate(turn, (-1, 0));
            }
            '>' => {
                switcher.mutate_coordinate(turn, (1, 0));
            }
            '^' => {
                switcher.mutate_coordinate(turn, (0, 1));
            }
            'v' => {
                switcher.mutate_coordinate(turn, (0, -1));
            }
            _ => {}
        }

        let next_coordinate = switcher.get_coordinate(turn);
        let next_coordinate_visits = map.get(&next_coordinate);
        if next_coordinate_visits.is_none() {
            at_least_once += 1;
            map.insert(next_coordinate, 1);
        }
        turn += 1;
    }

    at_least_once.to_string()
}
