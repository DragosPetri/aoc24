use std::{collections::HashSet, fs, iter, usize};

#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn outside_map(map: &[Vec<char>], x_coord: i32, y_coord: i32) -> bool {
    if x_coord == -1
        || y_coord == -1
        || x_coord == map[0].len() as i32
        || y_coord == map.len() as i32
    {
        return true;
    }

    false
}

fn make_move(direction: &Direction, x: i32, y: i32) -> (i32, i32) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn valid_move(map: &[Vec<char>], direction: &Direction, x: i32, y: i32) -> Option<bool> {
    match direction {
        Direction::Up => {
            let x_up = x;
            let y_up = y - 1;

            if outside_map(map, x_up, y_up) {
                None
            } else if map[y_up as usize][x_up as usize] == '#' {
                return Some(false);
            } else {
                return Some(true);
            }
        }
        Direction::Down => {
            let x_down = x;
            let y_down = y + 1;

            if outside_map(map, x_down, y_down) {
                None
            } else if map[y_down as usize][x_down as usize] == '#' {
                return Some(false);
            } else {
                return Some(true);
            }
        }
        Direction::Left => {
            let x_left = x - 1;
            let y_left = y;

            if outside_map(map, x_left, y_left) {
                None
            } else if map[y_left as usize][x_left as usize] == '#' {
                return Some(false);
            } else {
                return Some(true);
            }
        }
        Direction::Right => {
            let x_right = x + 1;
            let y_right = y;

            if outside_map(map, x_right, y_right) {
                None
            } else if map[y_right as usize][x_right as usize] == '#' {
                return Some(false);
            } else {
                return Some(true);
            }
        }
    }
}

fn turn_until_valid(map: &[Vec<char>], direction: &Direction, x: i32, y: i32) -> Option<Direction> {
    let mut new_direction = direction.clone();

    loop {
        let is_valid = valid_move(map, &new_direction, x, y);

        match is_valid {
            Some(valid) => {
                if valid {
                    return Some(new_direction);
                } else {
                    new_direction = match new_direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    }
                }
            }
            None => return None,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/6/input.txt").expect("lmao");

    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut x = -1;
    let mut y = -1;

    'outer: for (y_index, row) in map.iter().enumerate() {
        for (x_index, el) in row.iter().enumerate() {
            if *el == '^' {
                x = x_index as i32;
                y = y_index as i32;
                break 'outer;
            }
        }
    }

    if x == -1 || y == -1 {
        println!("lmao no start");
        return;
    }

    let mut direction: Direction = Direction::Up;
    let mut visited_positions: HashSet<(u32, u32)> = HashSet::new();

    let init_x = x;
    let init_y = y;
    visited_positions.insert((x as u32, y as u32));

    loop {
        let maybe_valid_move = valid_move(&map, &direction, x, y);

        match maybe_valid_move {
            Some(is_valid) => {
                if !is_valid {
                    let maybe_turn = turn_until_valid(&map, &direction, x, y);

                    match maybe_turn {
                        Some(new_direction) => direction = new_direction,
                        None => break,
                    }
                }
                let (new_x, new_y) = make_move(&direction, x, y);
                x = new_x;
                y = new_y;
                visited_positions.insert((x as u32, y as u32));
            }
            None => break,
        }
    }

    println!("{}", visited_positions.len());

    visited_positions.remove(&(init_x as u32, init_y as u32));

    let mut loops = 0;

    for (v_x, v_y) in visited_positions {
        let mut new_map = map.clone();
        x = init_x;
        y = init_y;
        direction = Direction::Up;

        new_map[v_y as usize][v_x as usize] = '#';

        let mut new_visited_positions: HashSet<(u32, u32, Direction)> = HashSet::new();
        new_visited_positions.insert((x as u32, y as u32, direction.clone()));

        loop {
            let maybe_valid_move = valid_move(&new_map, &direction, x, y);

            match maybe_valid_move {
                Some(is_valid) => {
                    if !is_valid {
                        let maybe_turn = turn_until_valid(&new_map, &direction, x, y);

                        match maybe_turn {
                            Some(new_direction) => direction = new_direction,
                            None => break,
                        }
                    }
                    let (new_x, new_y) = make_move(&direction, x, y);
                    x = new_x;
                    y = new_y;
                    let ok = new_visited_positions.insert((x as u32, y as u32, direction.clone()));
                    if !ok {
                        loops += 1;
                        break;
                    }
                }
                None => break,
            }
        }
    }

    println!("{}", loops);
}
