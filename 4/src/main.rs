use std::{fs, str::from_utf8};

fn main() {
    let input = fs::read_to_string("./inputs/4/input.txt").expect("lmao");

    let horizontal_xmas: usize = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .windows(4)
                .filter(|window| {
                    let word = from_utf8(window).unwrap();
                    word == "XMAS" || word == "SAMX"
                })
                .count()
        })
        .sum();

    let input2d: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let input2d_rev: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().rev().collect())
        .collect();

    let count_xmas = |input: Vec<Vec<char>>| -> usize {
        input
            .iter()
            .map(|line| {
                line.windows(4)
                    .filter(|window| {
                        let word = window.iter().collect::<String>();
                        word == "XMAS" || word == "SAMX"
                    })
                    .count()
            })
            .sum::<usize>()
    };

    let mut vertical_input2d: Vec<Vec<char>> = vec![];

    (0..input2d[0].len()).for_each(|j| {
        let mut new_row: Vec<char> = vec![];

        (0..input2d.len()).for_each(|i| {
            new_row.push(input2d[i][j]);
        });

        vertical_input2d.push(new_row);
    });

    let mut l_r_diagonal_input2d_top: Vec<Vec<char>> = vec![];
    let mut l_r_diagonal_input2d_bottom: Vec<Vec<char>> = vec![];

    (0..input2d.len()).for_each(|i| {
        let mut top_row: Vec<char> = vec![];
        let mut bottom_row: Vec<char> = vec![];
        let mut k = i;
        (0..=i).for_each(|j| {
            if j < input2d[0].len() {
                if i == input2d.len() - 1 {
                    top_row.push(input2d[k][j]);
                } else {
                    top_row.push(input2d[k][j]);
                    bottom_row.push(input2d[input2d.len() - 1 - k][input2d[0].len() - 1 - j]);
                }
                k = k.saturating_sub(1);
            }
        });
        l_r_diagonal_input2d_top.push(top_row);
        l_r_diagonal_input2d_bottom.push(bottom_row);
    });

    l_r_diagonal_input2d_top.extend(l_r_diagonal_input2d_bottom);

    let mut r_l_diagonal_input2d_top: Vec<Vec<char>> = vec![];
    let mut r_l_diagonal_input2d_bottom: Vec<Vec<char>> = vec![];

    (0..input2d_rev.len()).for_each(|i| {
        let mut top_row: Vec<char> = vec![];
        let mut bottom_row: Vec<char> = vec![];
        let mut k = i;
        (0..=i).for_each(|j| {
            if j < input2d_rev[0].len() {
                if i == input2d_rev.len() - 1 {
                    top_row.push(input2d_rev[k][j]);
                } else {
                    top_row.push(input2d_rev[k][j]);
                    bottom_row
                        .push(input2d_rev[input2d_rev.len() - 1 - k][input2d_rev[0].len() - 1 - j]);
                }
                k = k.saturating_sub(1);
            }
        });
        r_l_diagonal_input2d_top.push(top_row);
        r_l_diagonal_input2d_bottom.push(bottom_row);
    });

    r_l_diagonal_input2d_top.extend(r_l_diagonal_input2d_bottom);

    let l_r_diag_xmas: usize = count_xmas(l_r_diagonal_input2d_top);
    let r_l_diag_xmas: usize = count_xmas(r_l_diagonal_input2d_top);
    let vertical_xmas: usize = count_xmas(vertical_input2d);

    println!("{:?}", horizontal_xmas);
    println!("{:?}", vertical_xmas);
    println!("{:?}", l_r_diag_xmas);
    println!("{:?}", r_l_diag_xmas);
    println!(
        "{:?}",
        horizontal_xmas + vertical_xmas + l_r_diag_xmas + r_l_diag_xmas
    );

    let mut mas_count = 0;

    (1..input2d.len() - 1).for_each(|i| {
        (1..input2d[0].len() - 1).for_each(|j| {
            let diag1 = [input2d[i - 1][j - 1], input2d[i][j], input2d[i + 1][j + 1]]
                .iter()
                .collect::<String>();

            let diag2 = [input2d[i - 1][j + 1], input2d[i][j], input2d[i + 1][j - 1]]
                .iter()
                .collect::<String>();

            if (diag1 == "MAS" || diag1 == "SAM") && (diag2 == "MAS" || diag2 == "SAM") {
                mas_count += 1;
            }
        });
    });

    println!("{}", mas_count);
}
