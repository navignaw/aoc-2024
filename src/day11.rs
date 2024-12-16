use std::collections::HashMap;

type StoneCounter = HashMap<u64, usize>;

fn update_counter(stones: &StoneCounter) -> StoneCounter {
    let new_counter = stones
        .into_iter()
        .fold(StoneCounter::new(), |mut acc, (stone, count)| {
            let num_digits: u32 = stone.checked_ilog10().unwrap_or(0) + 1;
            fn add_counter(counter: &mut StoneCounter, entry: u64, count: &usize) {
                counter
                    .entry(entry)
                    .and_modify(|s| *s += *count)
                    .or_insert(*count);
            }
            match (stone, num_digits % 2) {
                // base case
                (0, _) => add_counter(&mut acc, 1, count),
                (_, 0) => {
                    // even number of digits
                    let base: u64 = 10u64.pow(num_digits / 2);
                    add_counter(&mut acc, stone / base, count);
                    add_counter(&mut acc, stone % base, count);
                }
                _ => add_counter(&mut acc, stone * 2024, count),
            };
            acc
        });
    new_counter
}

fn part1(stone_counter: &StoneCounter) {
    let mut new_stones = stone_counter.clone();
    for _ in 0..25 {
        new_stones = update_counter(&new_stones);
    }
    println!("Num stones: {}", new_stones.into_values().sum::<usize>());
}

fn part2(stone_counter: &StoneCounter) {
    let mut new_stones = stone_counter.clone();
    for _ in 0..75 {
        new_stones = update_counter(&new_stones);
    }
    println!("Num stones: {}", new_stones.into_values().sum::<usize>());
}

pub fn main(input: String) {
    let stones: Vec<u64> = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let stone_counter: StoneCounter = stones.iter().fold(StoneCounter::new(), |mut acc, stone| {
        acc.entry(*stone).and_modify(|s| *s += 1).or_insert(1);
        acc
    });

    part1(&stone_counter);
    part2(&stone_counter);
}
