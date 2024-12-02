use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/2/input.txt").expect("lmao");

    let safe_report = |input: &[u32]| -> bool {
        input
            .windows(2)
            .all(|window| (1..=3).contains(&u32::abs_diff(window[0], window[1])))
            && (input.is_sorted() || input.is_sorted_by(|a, b| a > b))
    };

    let safe_count: u32 = input
        .lines()
        .map(|line| {
            let values = line
                .split_whitespace()
                .map(|nr| nr.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if safe_report(&values) {
                1
            } else {
                // Part 2
                for i in 0..values.len() {
                    let mut partial = values[0..i].to_owned();
                    partial.extend(values[i + 1..values.len()].to_owned());
                    if safe_report(&partial) {
                        return 1;
                    }
                }
                0
            }
        })
        .sum();

    println!("{}", safe_count);
}
