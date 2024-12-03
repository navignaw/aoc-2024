use itertools::Itertools;

// Returns a list of differences.
fn check_report(report: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|slice| slice[0] - slice[1]).collect();
    return diffs.iter().all(|num| (1..=3).contains(num))
        || diffs.iter().all(|num| (-3..=-1).contains(num));
}

fn part1(reports: &Vec<Vec<i32>>) {
    let num_safe: i32 = reports
        .into_iter()
        .map(|report| {
            // Map each report to whether it's safe or unsafe.
            let is_safe = check_report(report);
            return if is_safe { 1 } else { 0 };
        })
        .sum();

    println!("Num safe: {}", num_safe)
}

fn part2(reports: &Vec<Vec<i32>>) {
    let num_safe: i32 = reports
        .into_iter()
        .map(|report| {
            let k = report.len();
            // For every subset of length k-1, check if it's safe or unsafe.
            let any_safe: bool = report
                .clone()
                .into_iter()
                .combinations(k - 1)
                .any(|cleaned_report| check_report(&cleaned_report));
            return if any_safe { 1 } else { 0 };
        })
        .sum();

    println!("Num safe: {}", num_safe)
}

pub fn main(input: String) {
    // Parse the input
    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect();

    part1(&reports);
    part2(&reports);
}
