type Stones = Vec<u64>;

fn iterate_stones(stones: &Stones) -> Stones {
    let new_stones = stones.into_iter().fold(Stones::new(), |mut acc, stone| {
        let num_digits: u32 = stone.checked_ilog10().unwrap_or(0) + 1;
        match (stone, num_digits % 2) {
            // base case
            (0, _) => acc.push(1),
            (_, 0) => {
                // even number of digits
                let base: u64 = 10u64.pow(num_digits / 2);
                acc.push(stone / base);
                acc.push(stone % base);
            }
            _ => acc.push(stone * 2024),
        };
        acc
    });
    new_stones
}

fn part1(stones: &Stones) {
    let mut new_stones = stones.clone();
    for _ in 0..25 {
        new_stones = iterate_stones(&new_stones);
    }
    println!("Num stones: {}", new_stones.len());
}

fn part2(stones: &Stones) {
    let mut new_stones = stones.clone();
    for _ in 0..75 {
        new_stones = iterate_stones(&new_stones);
    }
    println!("Num stones: {}", new_stones.len());
}

pub fn main(input: String) {
    let stones: Stones = input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    part1(&stones);
    part2(&stones);
}
