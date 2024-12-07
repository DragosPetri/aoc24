use std::fs;

use itertools::Itertools;

struct Equation {
    result: u64,
    operands: Vec<u64>,
}

fn main() {
    let input = fs::read_to_string("./inputs/7/input.txt").expect("lmao");

    let equations = input
        .lines()
        .map(|line| {
            let result = line
                .split_whitespace()
                .find(|el| el.contains(":"))
                .unwrap()
                .trim_end_matches(":")
                .parse::<u64>()
                .unwrap();

            let operands = line
                .split_whitespace()
                .filter(|el| !el.contains(":"))
                .map(|el| el.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();

            Equation { result, operands }
        })
        .collect::<Vec<Equation>>();

    let sum: u64 = equations
        .iter()
        .map(|equation| {
            let operators = std::iter::repeat("+*|".chars())
                .take(equation.operands.len() - 1)
                .multi_cartesian_product();

            for x in operators {
                let mut possible_sum = equation.operands[0];

                for index in 1..equation.operands.len() {
                    match x[index - 1] {
                        '+' => possible_sum += equation.operands[index],
                        '*' => possible_sum *= equation.operands[index],
                        '|' => {
                            possible_sum = 10u64
                                .pow(equation.operands[index].checked_ilog10().unwrap_or(0) + 1)
                                * possible_sum
                                + equation.operands[index];
                        }
                        _ => todo!(),
                    }
                }

                if possible_sum == equation.result {
                    return possible_sum;
                }
            }
            0
        })
        .sum();

    println!("{}", sum);
}
