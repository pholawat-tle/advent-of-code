use advent_of_code::read_txt;

fn main() {
    let input = read_txt("d2_2015");
    let result = part1(&input);
    println!("{}", result);
    let result = part2(&input);
    println!("{}", result);
}

fn part1(input: &str) -> String {
    let presents = input.lines();

    presents
        .fold(0, |total_required, present| {
            let dimensions_str = present.split("x");
            let dimensions_str: Vec<&str> = dimensions_str.collect();

            let dimensions: Vec<u32> = dimensions_str
                .into_iter()
                .map(|side| side.parse::<u32>().expect("unable to parse side into u32"))
                .collect();

            let l = dimensions.get(0).unwrap();
            let w = dimensions.get(1).unwrap();
            let h = dimensions.get(2).unwrap();

            let side_1 = l * w;
            let side_2 = w * h;
            let side_3 = h * l;

            let min_side = side_1.min(side_2);
            let min_side = min_side.min(side_3);

            let wrapping_paper_size = (2 * side_1) + (2 * side_2) + (2 * side_3) + min_side;

            total_required + wrapping_paper_size
        })
        .to_string()
}

fn part2(input: &str) -> String {
    let presents = input.lines();

    presents
        .fold(0, |total_required, present| {
            let dimensions_str = present.split("x");
            let dimensions_str: Vec<&str> = dimensions_str.collect();

            let mut dimensions: Vec<u32> = dimensions_str
                .into_iter()
                .map(|side| side.parse::<u32>().expect("unable to parse side into u32"))
                .collect();

            dimensions.sort();

            let l = dimensions.get(0).unwrap();
            let w = dimensions.get(1).unwrap();
            let h = dimensions.get(2).unwrap();

            let side_1 = l + w;

            let perimeter = 2 * side_1;
            let bow_size = l * w * h;

            total_required + perimeter + bow_size
        })
        .to_string()
}
