use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn parse_page_orders(input: &str) -> HashMap<u32, HashSet<u32>> {
    let page_orders: Vec<(u32, u32)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once("|").unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    return page_orders
        .into_iter()
        .fold(HashMap::new(), |mut hash_map, (a, b)| {
            match hash_map.get_mut(&a) {
                Some(set) => {
                    set.insert(b);
                }
                None => {
                    let mut set = HashSet::new();
                    set.insert(b);
                    hash_map.insert(a, set);
                }
            };
            hash_map
        });
}

fn exists_path(
    a: u32,
    b: u32,
    page_orders: &HashMap<u32, HashSet<u32>>,
    cache: &mut HashMap<u32, bool>,
) -> bool {
    match cache.get(&a) {
        Some(res) => *res,
        None => match page_orders.get(&a) {
            None => {
                cache.insert(a, false);
                false
            }
            Some(values) => values.into_iter().any(|value| {
                let res = *value == b/* || exists_path(*value, b, page_orders, cache)*/;
                cache.insert(a, res);
                res
            }),
        },
    }
}

fn is_ordered(pages: &Vec<u32>, page_orders: &HashMap<u32, HashSet<u32>>) -> bool {
    // Pages are ordered if for every pair (a,b), there does not exist a path b -> a.
    let mut cache: HashMap<u32, bool> = HashMap::new();
    return !pages
        .into_iter()
        .combinations(2)
        .any(|pair| exists_path(*pair[1], *pair[0], page_orders, &mut cache));
}

fn part1(page_orders: &HashMap<u32, HashSet<u32>>, pages: &Vec<Vec<u32>>) {
    let middle_pages: Vec<&u32> = pages
        .into_iter()
        .filter_map(|line| {
            if is_ordered(&line, page_orders) {
                let n = line.clone().len();
                return line.get(n / 2);
            }
            return None;
        })
        .collect();
    let sum: u32 = middle_pages.into_iter().sum();
    println!("Sum of middle pages: {}", sum);
}

fn part2(page_orders: &HashMap<u32, HashSet<u32>>, pages: &Vec<Vec<u32>>) {}

pub fn main(input: String) {
    let (page_orders_str, pages_str) = input.split_once("\n\n").unwrap();
    let page_orders = parse_page_orders(page_orders_str);
    println!("page orders: {:?}", page_orders);
    let pages: Vec<Vec<u32>> = pages_str
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    part1(&page_orders, &pages);
    part2(&page_orders, &pages);
}
