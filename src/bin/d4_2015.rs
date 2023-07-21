use advent_of_code::read_txt;
use md5::Digest;

fn main() {
    let input = read_txt("d4_2015");
    let result = part1(&input.trim());
    println!("{}", result);
    let result = part2(&input.trim());
    println!("{}", result);
}

fn part1(input: &str) -> String {
    let mut number = 0;

    loop {
        let digest = compute_md5(input, number);
        let sum = digest.0[0] as i32 + digest.0[1] as i32 + (digest.0[2] >> 4) as i32;

        if sum == 0 {
            return number.to_string();
        }
        number += 1;
    }
}

fn part2(input: &str) -> String {
    let mut number = 0;

    loop {
        let digest = compute_md5(input, number);
        let sum = digest.0[0] as i32 + digest.0[1] as i32 + digest.0[2] as i32;

        if sum == 0 {
            return number.to_string();
        }
        number += 1;
    }
}

fn compute_md5(input: &str, number: u32) -> Digest {
    let concat_str = format!("{}{}", input, number);
    md5::compute(concat_str.as_bytes())
}
