use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/1/input.txt").expect("lmao");

    let (left, right): (Vec<(usize, u64)>, Vec<(usize, u64)>) = input
        .split_whitespace()
        .map(|el| el.parse::<u64>().unwrap())
        .enumerate()
        .partition(|(index, _)| index % 2 != 0);

    let mut x = left.into_iter().map(|(_, el)| el).collect::<Vec<u64>>();
    x.sort();
    let mut y = right.into_iter().map(|(_, el)| el).collect::<Vec<u64>>();
    y.sort();

    let z: u64 = x
        .iter()
        .zip(&y)
        .map(|(a, b)| u64::abs_diff(*a, *b))
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    let r: u64 = x
        .iter()
        .map(|a| *a * y.iter().filter(|b| *b == a).count() as u64)
        .collect::<Vec<u64>>()
        .iter()
        .sum();

    println!("{:?}", z);
    println!("{:?}", r);
}
