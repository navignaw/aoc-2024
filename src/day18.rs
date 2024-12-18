use std::collections::{HashMap, HashSet};

type Position = (i32, i32);
// Mapping from x,y position to weight (distance to that node).
type Grid = HashMap<Position, i32>;

const GRID_SIZE: i32 = 71;

fn compute_neighbors((x, y): Position) -> Vec<Position> {
    return [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|(a, b)| *a >= 0 && *b >= 0 && *a < GRID_SIZE && *b < GRID_SIZE)
        .collect();
}

fn print_grid(grid: &Grid) {
    println!("GRID WEIGHTS:");
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            print!(
                "{}",
                grid.get(&(x, y)).map_or_else(
                    || String::from("?"),
                    |weight| if *weight == i32::MAX {
                        String::from("#")
                    } else {
                        (*weight % 10).to_string()
                    }
                )
            );
        }
        print!("\n");
    }
}

fn compute_weight(grid: &mut Grid) -> Option<i32> {
    // Initialize frontier with the final exit node
    let mut frontier: HashSet<Position> = HashSet::from([(GRID_SIZE - 1, GRID_SIZE - 1)]);
    let mut final_weight: Option<i32> = None;
    for weight in 0..i32::MAX {
        frontier = frontier.iter().fold(HashSet::new(), |mut acc, node| {
            let new_neighbors = compute_neighbors(*node);
            new_neighbors.iter().for_each(|neighbor| {
                // Add 1 to each neighbor, if it isn't already in the grid
                if !grid.contains_key(neighbor) {
                    grid.insert(*neighbor, weight + 1);
                    acc.insert(*neighbor);
                }
            });
            acc
        });
        //print_grid(&grid);

        // Check if we have the final weight
        match grid.get(&(0, 0)) {
            Some(weight) => {
                final_weight = Some(*weight);
                break;
            }
            None => (),
        };
        if frontier.len() == 0 {
            break;
        }
    }
    return final_weight;
}

fn part1(input: &String) {
    let mut grid = input_to_grid(input, 1024);
    match compute_weight(&mut grid) {
        Some(final_weight) => println!("Path length: {}", final_weight),
        None => println!("No path achievable"),
    };
}

fn part2(input: &String) {
    for num_failures in 1024..usize::MAX {
        let mut grid = input_to_grid(input, num_failures);
        match compute_weight(&mut grid) {
            Some(_) => println!("Found path with {} failures", num_failures),
            None => {
                println!("No path achievable with {} failures", num_failures);
                println!(
                    "Position of failure: {}",
                    input.lines().collect::<Vec<&str>>()[num_failures - 1]
                );
                break;
            }
        };
    }
}

fn input_to_grid(input: &String, num_failures: usize) -> Grid {
    let mut init_grid = Grid::new();
    init_grid.insert((GRID_SIZE - 1, GRID_SIZE - 1), 1);

    return input
        .lines()
        .enumerate()
        .fold(init_grid, |mut acc, (i, pos)| {
            match (i < num_failures, pos.split_once(",")) {
                (true, Some((xs, ys))) => {
                    let x: i32 = xs.parse().unwrap();
                    let y: i32 = ys.parse().unwrap();
                    acc.insert((x, y), i32::MAX);
                    acc
                }
                (false, _) => acc,
                _ => panic!("Invalid input"),
            }
        });
}

pub fn main(input: String) {
    part1(&input);
    part2(&input);
}
