use std::collections::HashMap;

fn part1(a: &Vec<i32>, b: &Vec<i32>) {
    // Sort the numbers in both a and b
    let mut a_sorted = a.to_vec();
    a_sorted.sort();
    let mut b_sorted = b.to_vec();
    b_sorted.sort();

    // For each value, take the difference between the max and min
    let total_diff: u32 = a_sorted
        .iter()
        .zip(b_sorted.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("Total diff: {}", total_diff);
}

fn counter(nums: &Vec<i32>) -> HashMap<i32, i32> {
    return nums.iter().fold(HashMap::new(), |mut counter, num| {
        *counter.entry(*num).or_insert(0) += 1;
        return counter;
    });
}

fn part2(a: &Vec<i32>, b: &Vec<i32>) {
    // Compute a counter of the number of times each number appears
    let counter_b = counter(b);
    let total_similarity: i32 = a
        .iter()
        .map(|num| {
            // Look up the number in the right list
            let count = counter_b.get(num).unwrap_or(&0);
            return num * count;
        })
        .sum();
    println!("Total similarity: {}", total_similarity);
}

pub fn main(input: String) {
    // Parse the input
    let (a, b): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let nums: Vec<i32> = line
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    part1(&a, &b);
    part2(&a, &b);
}
