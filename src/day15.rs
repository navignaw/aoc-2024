use core::panic;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone)]
enum Object {
    ROBOT,
    BOX,
    WALL,
}
#[derive(Debug)]
enum Move {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

const GRID_SIZE: i32 = 50;

type Position = (i32, i32);
type GridMap = HashMap<Position, Object>;
struct Grid<'a> {
    map: &'a mut GridMap,
    robot: Option<Position>,
}
impl<'a> fmt::Display for Grid<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                write!(
                    f,
                    "{}",
                    match self.map.get(&(i, j)) {
                        Some(Object::ROBOT) => '@',
                        Some(Object::BOX) => 'O',
                        Some(Object::WALL) => '#',
                        None => '.',
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl<'a> Grid<'a> {
    fn new(map: &'a mut GridMap) -> Grid {
        Grid { map, robot: None }
    }

    fn set_robot(&mut self, position: Position) {
        self.robot = Some(position);
    }

    // Returns a list of positions from the robot's position, up to an empty space.
    fn _get_spaces(&self, mov: &Move) -> Option<Vec<Position>> {
        let (i, j) = self.robot.unwrap();
        //println!("robot is at: {}, {}", i, j);
        let range: Vec<Position> = match *mov {
            Move::UP => (0..=i).rev().map(|row| (row, j)).collect(),
            Move::DOWN => (i..GRID_SIZE).map(|row| (row, j)).collect(),
            Move::LEFT => (0..=j).rev().map(|col| (i, col)).collect(),
            Move::RIGHT => (j..GRID_SIZE).map(|col| (i, col)).collect(),
        };
        let spaces_to_empty: Vec<Position> = range
            .into_iter()
            // Keep taking positions until we find one missing from the map (empty space)
            .map_while(|pos| self.map.get(&pos).map(|_| pos))
            .collect();

        // If any position is a wall, we can't push
        if spaces_to_empty.iter().any(|pos| match self.map.get(&pos) {
            Some(Object::WALL) => true,
            _ => false,
        }) {
            return None;
        }
        return Some(spaces_to_empty);
    }

    fn apply_move(&mut self, mov: &Move) -> &Grid {
        //println!("Applying move: {:?}", mov);
        let diff: (i32, i32) = match *mov {
            Move::UP => (-1, 0),
            Move::DOWN => (1, 0),
            Move::LEFT => (0, -1),
            Move::RIGHT => (0, 1),
        };
        match self._get_spaces(mov) {
            None => (), // Do nothing if we can't move
            Some(positions) => {
                //println!("found positions: {:?}", positions);
                // For each position in reverse order, move the object over
                positions.iter().rev().for_each(|&(i, j)| {
                    let obj = self.map.get(&(i, j)).unwrap().clone();
                    self.map.insert((i + diff.0, j + diff.1), obj.clone());
                    // Update the robot's position and clear the old spot
                    if matches!(obj, Object::ROBOT) {
                        self.set_robot((i + diff.0, j + diff.1));
                        self.map.remove(&(i, j));
                    }
                });
            }
        };
        //println!("New board:\n{}", self);
        self
    }
}

fn compute_gps(grid: &Grid) -> i32 {
    return grid
        .map
        .iter()
        .filter_map(|((i, j), obj)| match obj {
            Object::BOX => Some(i * 100 + j),
            _ => None,
        })
        .sum();
}

fn part1(grid_str: &str, move_str: &str) {
    let mut hash_map = GridMap::new();
    let (mut grid, moves) = (parse_grid(grid_str, &mut hash_map), parse_moves(move_str));

    let output_grid = moves.into_iter().fold(&mut grid, |acc: &mut Grid, mov| {
        acc.apply_move(&mov);
        acc
    });
    let gps_sum = compute_gps(&output_grid);
    println!("Total GPS: {}", gps_sum);
}

fn part2(grid_str: &str, move_str: &str) {
    let gps_sum = 0;
    println!("Total GPS: {}", gps_sum);
}

fn parse_grid<'a>(grid_str: &str, hash_map: &'a mut GridMap) -> Grid<'a> {
    let initial_grid = Grid::new(hash_map);
    return grid_str
        .lines()
        .enumerate()
        .fold(initial_grid, |grid, (i, line)| {
            line.chars().enumerate().fold(grid, |mut acc, (j, char)| {
                let pos = (i.try_into().unwrap(), j.try_into().unwrap());
                match char {
                    '@' => {
                        acc.set_robot(pos);
                        acc.map.insert(pos, Object::ROBOT)
                    }
                    '#' => acc.map.insert(pos, Object::WALL),
                    'O' => acc.map.insert(pos, Object::BOX),
                    _ => None,
                };
                acc
            })
        });
}

fn parse_moves(move_str: &str) -> Vec<Move> {
    return move_str
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Move::UP),
            'v' => Some(Move::DOWN),
            '>' => Some(Move::RIGHT),
            '<' => Some(Move::LEFT),
            _ => None,
        })
        .collect();
}

pub fn main(input: String) {
    match input.split_once("\n\n") {
        Some((grid_str, move_str)) => {
            part1(grid_str, move_str);
            part2(grid_str, move_str);
        }
        None => panic!("Invalid input"),
    };
}
