use regex::Regex;

fn apply_mul(input: &String) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    return re
        .captures_iter(input)
        .map(|c| {
            let (_, [d1, d2]) = c.extract();
            return d1.parse::<u64>().unwrap() * d2.parse::<u64>().unwrap();
        })
        .sum();
}

fn part1(input: &String) {
    let sum = apply_mul(input);
    println!("Sum of muls: {}", sum)
}

fn part2(input: &String) {
    // Split strings on don't()
    let v: Vec<&str> = input.split("don't()").collect();
    let sum: u64 = match v.split_first() {
        None => 0,
        Some((head, rest)) => {
            // Trim everything up to the first do()
            let trimmed: Vec<String> = rest
                .into_iter()
                .map(|s| {
                    s.split("do()")
                        .into_iter()
                        .skip(1)
                        .collect::<Vec<&str>>()
                        .join("")
                })
                .collect();
            let head_str: &String = &(*head).to_owned();
            apply_mul(head_str) + trimmed.into_iter().map(|s| apply_mul(&s)).sum::<u64>()
        }
    };
    println!("Sum of enabled muls: {}", sum);
}

pub fn main(input: String) {
    part1(&input);
    part2(&input);
}
