use std::fs;

#[derive(Debug)]
struct Item {
    nr: u32,
    smol: Vec<u32>,
    big: Vec<u32>,
}

fn main() {
    let input = fs::read_to_string("./inputs/5/input.txt").expect("lmao");

    let rules: Vec<&str> = input.lines().filter(|line| line.contains("|")).collect();
    let updates: Vec<Vec<u32>> = input
        .lines()
        .filter(|line| line.contains(","))
        .map(|line| {
            line.split(",")
                .map(|nr| nr.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    let mut items: Vec<Item> = vec![];

    for rule in &rules {
        let mut nrs = rule.split("|");
        let lhs = nrs.next().unwrap().parse::<u32>().unwrap();
        let rhs = nrs.next().unwrap().parse::<u32>().unwrap();

        if let Some(index) = items.iter().position(|item| item.nr == lhs) {
            items[index].big.push(rhs);

            if let Some(index_of_rhs) = items.iter().position(|item| item.nr == rhs) {
                items[index_of_rhs].smol.push(lhs);
            } else {
                items.push(Item {
                    nr: rhs,
                    smol: vec![lhs],
                    big: vec![],
                });
            }
        } else {
            items.push(Item {
                nr: lhs,
                smol: vec![],
                big: vec![rhs],
            });

            if let Some(index_of_rhs) = items.iter().position(|item| item.nr == rhs) {
                items[index_of_rhs].smol.push(lhs);
            } else {
                items.push(Item {
                    nr: rhs,
                    smol: vec![lhs],
                    big: vec![],
                });
            }
        }
    }

    let mut sum = 0;

    let is_sorted = |input: &Vec<u32>| -> bool {
        let mut ok = true;
        for (index, page) in input.iter().enumerate() {
            if let Some(rule_for_page) = items.iter().position(|item| item.nr == *page) {
                (0..index).for_each(|left| {
                    if items[rule_for_page].big.contains(&input[left]) {
                        ok = false;
                    }
                });
                (index + 1..input.len()).for_each(|right| {
                    if items[rule_for_page].smol.contains(&input[right]) {
                        ok = false;
                    }
                });
            }
        }

        ok
    };

    for update in &updates {
        if is_sorted(update) {
            sum += update[update.len() / 2];
        }
    }

    let is_bigger = |item: u32, comparator: u32| -> bool {
        let item_index = items.iter().position(|el| el.nr == item);

        let item_index = match item_index {
            Some(x) => x,
            None => return false,
        };

        items[item_index].big.contains(&comparator)
    };

    let newly_sorted_sum: u32 = updates
        .iter()
        .filter(|item| !is_sorted(item))
        .map(|unsorted| {
            let mut sorted: Vec<u32> = vec![];

            for item in unsorted {
                let index = sorted
                    .iter()
                    .position(|sorted_item| !is_bigger(*sorted_item, *item));
                match index {
                    Some(i) => {
                        sorted.insert(i, *item);
                    }
                    None => sorted.push(*item),
                }
            }

            sorted[sorted.len() / 2]
        })
        .sum();

    println!("{}", sum);
    println!("{}", newly_sorted_sum);
}
