use advent_of_code::read_txt;
use itertools::Itertools;

fn main() {
    let input = read_txt("d17_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

fn part1(input: &str) -> String {
    let containers: Vec<u32> = input
        .lines()
        .map(|x| x.parse().unwrap())
        .sorted()
        .collect_vec();

    let mut count = 0;
    for i in 1..(containers.len() + 1) {
        let mut combinations = containers.iter().combinations(i);
        while let Some(c) = combinations.next() {
            let sum = c.iter().fold(0, |sum, current| {
                sum + *current
            });

            if sum == 150 {
                count += 1;
            }
        }
    }
    
    format!("{} combinations", count)
}

fn part2(input: &str) -> String {
    let containers: Vec<u32> = input
        .lines()
        .map(|x| x.parse().unwrap())
        .sorted()
        .collect_vec();

    let mut min_container_used_option: Option<u32> = None;
    let mut count = 0;
    for i in 1..(containers.len() + 1) {
        let mut combinations = containers.iter().combinations(i);
        while let Some(c) = combinations.next() {
            let sum = c.iter().fold(0, |sum, current| {
                sum + *current
            });

            match min_container_used_option {
                Some(min_container_used) => {
                    if sum == 150 && c.len() == min_container_used as usize {
                        count += 1;
                    }

                    if sum == 150 && c.len() < min_container_used as usize {
                        count = 1;
                        min_container_used_option = Some(c.len() as u32);
                    }
                }
                None => {
                    if sum == 150 {
                        count += 1;
                        min_container_used_option = Some(c.len() as u32);
                    }
                }
            }
        }
    }
    
    format!("{} combinations", count)
}
