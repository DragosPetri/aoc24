use std::{fs, str::from_utf8};

fn main() {
    let input = fs::read_to_string("./inputs/3/input.txt").expect("lmao");

    let mut enabled = true;

    let sum: u64 = input
        .lines()
        .map(|line| {
            let padded_line = line.to_owned() + "000000000000";

            padded_line
                .as_bytes()
                .windows(12)
                .filter(|window| {
                    let word = from_utf8(window).unwrap();
                    if word.starts_with("do()") {
                        enabled = true;
                    } else if word.starts_with("don't()") {
                        enabled = false;
                    }
                    word.starts_with("mul(") && enabled
                })
                .map(|window| {
                    let word = from_utf8(window).unwrap();
                    word.trim_start_matches("mul(").to_owned()
                })
                .filter(|word| word.contains(")"))
                .map(|mut word| {
                    let index = word.chars().position(|c| c == ')').unwrap();
                    word.truncate(index);
                    word
                })
                .filter(|word| !word.contains(char::is_whitespace))
                .filter(|word| word.contains(","))
                .map(|word| {
                    if word.split(",").count() != 2 {
                        return 0;
                    }

                    let mut numbers = word.split(",");

                    let lhs = match numbers.next().unwrap().parse::<u64>() {
                        Ok(nr) => nr,
                        Err(_) => return 0,
                    };

                    let rhs = match numbers.next().unwrap().parse::<u64>() {
                        Ok(nr) => nr,
                        Err(_) => return 0,
                    };

                    lhs * rhs
                })
                .sum::<u64>()
        })
        .sum();

    println!("{:?}", sum);
}
