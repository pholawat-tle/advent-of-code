use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d18_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

const GRID_SIDE: i32 = 100;
type LightGrid = [[bool; 100]; 100];

fn parse_input(input: &str) -> LightGrid {
    let mut grid = [[false; GRID_SIDE as usize]; GRID_SIDE as usize];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let state = match c {
                '#' => true,
                '.' => false,
                _ => panic!("Unknown character"),
            };

            grid[y][x] = state;
        }
    }

    grid
}

fn step(previous_grid: LightGrid) -> LightGrid {
    let mut new_grid = [[false; 100]; 100];

    for y in 0..GRID_SIDE {
        for x in 0..GRID_SIDE {
            let previous_state = previous_grid[y as usize][x as usize];

            let neighbor_coordinates: Vec<(i32, i32)> = vec![
                (x - 1, y - 1),
                (x - 1, y),
                (x - 1, y + 1),
                (x, y - 1),
                (x, y + 1),
                (x + 1, y - 1),
                (x + 1, y),
                (x + 1, y + 1),
            ];

            let mut neighbor_on_count = 0;

            for neighbor in neighbor_coordinates {
                if neighbor.0 < 0
                    || neighbor.1 < 0
                    || neighbor.0 > GRID_SIDE - 1
                    || neighbor.1 > GRID_SIDE - 1
                {
                    continue;
                }

                match previous_grid[neighbor.1 as usize][neighbor.0 as usize] {
                    true => neighbor_on_count += 1,
                    false => (),
                }
            }

            match previous_state {
                true => {
                    if neighbor_on_count == 2 || neighbor_on_count == 3 {
                        new_grid[y as usize][x as usize] = true;
                    }
                }
                false => {
                    if neighbor_on_count == 3 {
                        new_grid[y as usize][x as usize] = true;
                    }
                }
            }
        }
    }

    new_grid
}

fn part1(input: &str) -> String {
    let mut grid = parse_input(input);

    for _ in 0..100 {
        grid = step(grid);
    }

    format!(
        "{:?}",
        grid.iter()
            .map(|x| x.iter().filter(|y| **y).count())
            .sum::<usize>()
    )
}

fn part2(input: &str) -> String {
    let mut grid = parse_input(input);

    for _ in 0..100 {
        grid = step(grid);

        grid[0][0] = true;
        grid[0][(GRID_SIDE - 1) as usize] = true;
        grid[(GRID_SIDE - 1) as usize][0] = true;
        grid[(GRID_SIDE - 1) as usize][(GRID_SIDE - 1) as usize] = true;
    }

    format!(
        "{:?}",
        grid.iter()
            .map(|x| x.iter().filter(|y| **y).count())
            .sum::<usize>()
    )
}
