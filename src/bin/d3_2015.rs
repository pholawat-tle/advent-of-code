use std::collections::HashSet;

use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d3_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

type Coordinate = (i32, i32);

struct TravelHistory {
    map: HashSet<Coordinate>,
}

impl TravelHistory {
    fn new() -> TravelHistory {
        let mut travel_history = TravelHistory {
            map: HashSet::new(),
        };
        travel_history.mark_visit((0, 0));
        travel_history
    }

    /// Returns true if the coordinate has never been visited before.
    fn mark_visit(&mut self, coordinate: Coordinate) -> bool {
        self.map.insert(coordinate)
    }
}

struct Scheduler {
    coordinates: Vec<Coordinate>,
    index: usize,
}

impl Scheduler {
    fn increment(&mut self) {
        self.index = (self.index + 1) % self.coordinates.len();
    }

    fn get_coordinate(&self) -> Coordinate {
        self.coordinates[self.index]
    }

    fn update_coordinate(&mut self, coordinate: Coordinate) {
        self.coordinates[self.index] = coordinate;
    }
}

fn part1(input: &str) -> String {
    let mut travel_history = TravelHistory::new();
    let orders = input.chars().into_iter();
    let mut current_coordinate = (0, 0);

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

        travel_history.mark_visit(current_coordinate);
    }

    travel_history.map.len().to_string()
}

fn part2(input: &str) -> String {
    let mut travel_history = TravelHistory::new();
    let orders = input.chars().into_iter();
    let starting_coordinate = (0, 0);

    let mut scheduler = Scheduler {
        coordinates: vec![starting_coordinate, starting_coordinate],
        index: 0,
    };

    for order in orders {
        let current_coordinate = scheduler.get_coordinate();
        let (mut x, mut y) = current_coordinate;

        match order {
            '<' => {
                x -= 1;
            }
            '>' => {
                x += 1;
            }
            '^' => {
                y += 1;
            }
            'v' => {
                y -= 1;
            }
            _ => {}
        }

        let new_coordinate = (x, y);

        scheduler.update_coordinate(new_coordinate);
        travel_history.mark_visit(new_coordinate);
        scheduler.increment();
    }

    travel_history.map.len().to_string()
}
