use itertools::Itertools;

fn xmas_count(input: &str) -> usize {
    return input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .filter_map(|slice| {
            let substring = slice.into_iter().join("");
            if substring == "XMAS" || substring == "SAMX" {
                Some(())
            } else {
                None
            }
        })
        .count();
}

fn part1(input: &String) {
    let rows = input.lines();
    let n: usize = rows.clone().count();
    let m: usize = rows.clone().nth(0).unwrap().len();
    let cols: Vec<String> = (0..m)
        .map(|i| {
            rows.clone()
                .into_iter()
                .map(|row| row.chars().nth(i).unwrap())
                .join("")
        })
        .collect();
    let diags: Vec<String> = (0..m - 1)
        .map(|i| {
            rows.clone()
                .into_iter()
                .enumerate()
                .filter_map(|(j, row)| row.chars().nth(i + j))
                .join("")
        })
        .collect();
    let rev_diags: Vec<String> = (0..m - 1)
        .map(|i| {
            rows.clone()
                .into_iter()
                .enumerate()
                .filter_map(|(j, row)| row.chars().nth_back(i + j))
                .join("")
        })
        .collect();
    let col_diags: Vec<String> = (1..n - 1)
        .map(|j| {
            cols.clone()
                .into_iter()
                .enumerate()
                .filter_map(|(i, col)| col.chars().nth(i + j))
                .join("")
        })
        .collect();
    let rev_col_diags: Vec<String> = (1..n - 1)
        .map(|j| {
            cols.clone()
                .into_iter()
                .rev()
                .enumerate()
                .filter_map(|(i, col)| col.chars().nth(i + j))
                .join("")
        })
        .collect();

    let count: usize = rows.map(|row| xmas_count(row)).sum::<usize>() +// Row count
        cols.into_iter().map(|col| xmas_count(&col)).sum::<usize>() + // Col count
        diags.into_iter().map(|diag| xmas_count(&diag)).sum::<usize>() +  // Diag count
        rev_diags.into_iter().map(|diag| xmas_count(&diag)).sum::<usize>() + // Reverse diag count
        col_diags.into_iter().map(|diag| xmas_count(&diag)).sum::<usize>() + // Col diag count
        rev_col_diags.into_iter().map(|diag| xmas_count(&diag)).sum::<usize>(); // Rev col diag count

    println!("Count of xmas: {}", count)
}

fn part2(input: &String) {
    println!("Not yet implemented");
}

pub fn main(input: String) {
    part1(&input);
    part2(&input);
}
