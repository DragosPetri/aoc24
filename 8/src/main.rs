use std::{fs, usize};

struct Antenna {
    freq: char,
    positions: Vec<(usize, usize)>,
}

fn main() {
    let input = fs::read_to_string("./inputs/8/input.txt").expect("lmao");

    let mut map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut antennas: Vec<Antenna> = vec![];

    for (y_index, row) in map.iter().enumerate() {
        for (x_index, el) in row.iter().enumerate() {
            if *el != '.' {
                let index = antennas.iter().position(|antenna| antenna.freq == *el);
                match index {
                    Some(pos) => {
                        antennas[pos].positions.push((x_index, y_index));
                    }
                    None => {
                        antennas.push(Antenna {
                            freq: *el,
                            positions: vec![(x_index, y_index)],
                        });
                    }
                }
            }
        }
    }

    let mut node_count = 0;

    for antenna in antennas {
        if antenna.positions.len() == 1 {
            continue;
        }

        for current_index in 0..antenna.positions.len() {
            for reference_index in 0..antenna.positions.len() {
                if reference_index == current_index {
                    continue;
                }

                let x_diff: i32 = antenna.positions[current_index].0 as i32
                    - antenna.positions[reference_index].0 as i32;
                let y_diff: i32 = antenna.positions[current_index].1 as i32
                    - antenna.positions[reference_index].1 as i32;

                let mut multiplier = 0;
                loop {
                    let antinode_x =
                        antenna.positions[current_index].0 as i32 - multiplier * x_diff;
                    let antinode_y =
                        antenna.positions[current_index].1 as i32 - multiplier * y_diff;

                    if !(0..map[0].len() as i32).contains(&antinode_x)
                        || !(0..map.len() as i32).contains(&antinode_y)
                    {
                        break;
                    }

                    if map[antinode_y as usize][antinode_x as usize] != '#' {
                        map[antinode_y as usize][antinode_x as usize] = '#';
                        node_count += 1;
                    }
                    multiplier += 1;
                }
            }
        }
    }

    for x in map {
        for y in x {
            print!("{}", y);
        }
        println!();
    }
    println!("{}", node_count);
}
