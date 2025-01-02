use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, usize,
};
mod point;
use point::*;

type Grid = Vec<Vec<char>>;

fn is_in_grid(pos: Point<i32>, grid: &Grid) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.x < grid.len() as i32 && pos.y < grid[0].len() as i32 {
        return true;
    }
    false
}

fn get_neighbors(grid: &Grid, pos: Point<i32>) -> Vec<Point<i32>> {
    let left = pos + Point::new(0, -1);
    let right = pos + Point::new(0, 1);
    let up = pos + Point::new(-1, 0);
    let down = pos + Point::new(1, 0);

    [left, right, up, down]
        .into_iter()
        .filter_map(|neighbor| {
            if is_in_grid(neighbor, grid) && grid[pos.x as usize][pos.y as usize] != '#' {
                return Some(neighbor);
            }
            None
        })
        .collect()
}

fn breadth_first_search(grid: &Grid, start: Point<i32>, end: Point<i32>) -> Option<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(start);

    let mut dist = HashMap::new();
    dist.insert(start, 0);

    while let Some(v) = queue.pop_front() {
        let neighbors = get_neighbors(grid, v);
        for neighbor in neighbors {
            if !dist.contains_key(&neighbor) {
                dist.insert(neighbor, dist[&v] + 1);

                if neighbor == end {
                    return Some(dist[&neighbor]);
                }

                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn print_grid(grid: &Grid, visited: &HashSet<Point<i32>>) {
    println!(
        "{}",
        grid.iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, c)| {
                        if visited.contains(&Point::new(j as i32, i as i32)) {
                            'O'
                        } else {
                            *c
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn init_grid(blocks: &Vec<Point<i32>>, n_bytes: usize) -> Grid {
    let mut grid: Grid = (0..=GRID_MAX)
        .map(|_| (0..=GRID_MAX).map(|_| '.').collect())
        .collect();

    for block in &blocks[..n_bytes] {
        grid[block.y as usize][block.x as usize] = '#';
    }

    grid
}

fn binary_search(blocks: &Vec<Point<i32>>, start: Point<i32>, end: Point<i32>) -> Point<i32> {
    let mut left = 0;
    let mut right = blocks.len();

    while left < right {
        let mid = (left + right) / 2;

        let grid = init_grid(blocks, mid);

        // want the leftmost element within the none space
        if breadth_first_search(&grid, start, end).is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    // subtract 1 for the index because mid was acting as the length of blocks
    blocks[left - 1]
}

// const GRID_MAX: usize = 6;
// const N_BYTES: usize = 12;
const GRID_MAX: usize = 70;
const N_BYTES: usize = 1024;
const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let blocks: Vec<Point<i32>> = input
        .lines()
        .map(|line| {
            let digits = line
                .split(",")
                .map(|v| v.parse().unwrap())
                .collect::<Vec<_>>();
            Point::new(digits[0], digits[1])
        })
        .collect();

    let grid = init_grid(&blocks, N_BYTES);
    let start = Point::new(0, 0);
    let end = Point::new(GRID_MAX as i32, GRID_MAX as i32);

    println!(
        "part 1: {}",
        breadth_first_search(&grid, start, end).unwrap()
    );

    println!("part 2: {:?}", binary_search(&blocks, start, end));
}
