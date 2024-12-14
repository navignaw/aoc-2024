type Equation = (u64, Vec<u64>);

fn concat_nums(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn check_equation(val: u64, nums: &Vec<u64>, include_concat: bool) -> bool {
    match (nums.len(), val) {
        (0, 0) => true,
        (0, _) => false,
        (1, _) => return val == nums[0],
        _ => {
            // Try each of the operators on the last two element
            let (head, tail) = nums.split_at(2);
            let (e1, e2) = (head[0], head[1]);
            let combined = if include_concat {
                Vec::from([e1 + e2, e1 * e2, concat_nums(e1, e2)])
            } else {
                Vec::from([e1 + e2, e1 * e2])
            };
            combined.into_iter().any(|num| {
                let mut new_nums = Vec::from([num]);
                new_nums.extend_from_slice(tail);
                return check_equation(val, &new_nums, include_concat);
            })
        }
    }
}

fn part1(equations: &Vec<Equation>) {
    let sum: u64 = equations
        .into_iter()
        .filter_map(|(val, nums)| {
            if check_equation(*val, nums, false) {
                Some(val)
            } else {
                None
            }
        })
        .sum();
    println!("Sum of equations: {}", sum)
}

fn part2(equations: &Vec<Equation>) {
    let sum: u64 = equations
        .into_iter()
        .filter_map(|(val, nums)| {
            if check_equation(*val, nums, true) {
                Some(val)
            } else {
                None
            }
        })
        .sum();
    println!("Sum of equations: {}", sum)
}

fn extract_equation(input: &str) -> Option<Equation> {
    input.split_once(": ").map(|(val, tail)| {
        (
            val.parse().unwrap(),
            tail.split(" ")
                .map(|num_str| num_str.parse().unwrap())
                .collect(),
        )
    })
}

pub fn main(input: String) {
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            return extract_equation(line).unwrap();
        })
        .collect();
    part1(&equations);
    part2(&equations);
}
