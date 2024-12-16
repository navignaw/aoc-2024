use itertools::{repeat_n, FoldWhile, Itertools};
use std::{collections::BTreeMap, convert::TryInto};

type DiskMap = Vec<Option<usize>>;
// Map from index to (id, size). Preserves sort by index.
type DiskHashMap = BTreeMap<usize, (Option<usize>, usize)>;

fn into_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

fn part1(disk_map: &DiskMap) {
    let n = disk_map.clone().len();
    let mut disk_map_array: [Option<usize>; 94796] = into_array(disk_map.clone());
    disk_map.into_iter().fold_while((0, n), |(i, j), val| {
        let j2 = match val {
            Some(_) => j,
            None => {
                // If missing, try moving the latest element
                let mut j2 = j;
                while j2 > i {
                    j2 -= 1;
                    if disk_map_array[j2] != None {
                        disk_map_array[i] = disk_map_array[j2];
                        disk_map_array[j2] = None;
                        break;
                    }
                }
                j2
            }
        };
        let acc = (i + 1, j2);
        return (if i + 1 >= j2 {
            FoldWhile::Done
        } else {
            FoldWhile::Continue
        })(acc);
    });
    let checksum = disk_map_array
        .iter()
        .enumerate()
        .fold(0, |acc, (i, val)| match val {
            Some(id) => acc + i * id,
            None => acc,
        });
    println!("Checksum: {}", checksum)
}

fn part2(disk_map: &DiskHashMap) {
    let reverse_map = disk_map.into_iter().rev();
    let compacted: DiskHashMap =
        reverse_map
            .clone()
            .fold(disk_map.clone(), |mut acc, (idx, (id, len))| match id {
                // If there is a value, attempt to move it to the earliest spot that fits
                Some(_) => {
                    let first_entry = acc.clone().into_iter().find(|val| {
                        let (idx2, (id2, len2)) = *val;
                        return id2 == None && len2 >= *len && idx2 < *idx;
                    });
                    match first_entry {
                        None => (),
                        Some((idx2, (id2, len2))) => {
                            // Move the entry
                            acc.insert(idx2, (*id, *len));
                            let diff_len = len2 - *len;
                            if diff_len > 0 {
                                acc.insert(idx2 + *len, (None, diff_len));
                            }
                            acc.remove_entry(idx);
                        }
                    }
                    acc
                }
                None => acc, // do nothing
            });
    let compacted_vec = compacted
        .into_iter()
        .fold(DiskMap::new(), |mut acc, (idx, (id, len))| {
            let n: usize = acc.len();
            if n < idx {
                acc.extend(repeat_n(None, idx - n));
            }
            acc.extend(repeat_n(id, len));
            acc
        });
    let checksum = compacted_vec
        .iter()
        .enumerate()
        .fold(0, |acc, (i, val)| match val {
            Some(id) => acc + i * id,
            None => acc,
        });
    println!("Checksum: {}", checksum)
}

fn extract_disk_map(input: String) -> DiskMap {
    input
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (i, c)| {
            let id = i / 2;
            let is_free_space = i % 2 == 1;
            match c.to_digit(10) {
                Some(n) => acc.extend(repeat_n(
                    if is_free_space { None } else { Some(id) },
                    n.try_into().unwrap(),
                )),
                None => (),
            }
            return acc;
        })
}

fn extract_disk_hashmap(input: String) -> DiskHashMap {
    let (disk_map, _) =
        input
            .chars()
            .enumerate()
            .fold((DiskHashMap::new(), 0), |(mut acc, idx), (i, c)| {
                let id = i / 2;
                let is_free_space = i % 2 == 1;
                match c.to_digit(10) {
                    Some(cn) => {
                        let n: usize = cn.try_into().unwrap();
                        acc.insert(idx, (if is_free_space { None } else { Some(id) }, n));
                        (acc, idx + n)
                    }
                    None => (acc, idx),
                }
            });
    disk_map
}

fn print_disk_map(disk_map: &DiskMap) {
    let disk_map_str: String = disk_map
        .clone()
        .into_iter()
        .map(|c| match c {
            None => '.',
            Some(v) => char::from_digit(v.try_into().unwrap(), 10).unwrap(),
        })
        .into_iter()
        .collect();
    println!("disk map: {}", disk_map_str);
}

pub fn main(input: String) {
    let disk_map: DiskMap = extract_disk_map(input.clone());
    let disk_hashmap: DiskHashMap = extract_disk_hashmap(input.clone());
    //print_disk_map(&disk_map)
    part1(&disk_map);
    part2(&disk_hashmap);
}
