use std::{collections::HashSet, fs, usize};
mod point;
use point::*;

type Grid = Vec<Vec<i32>>;

fn is_in_grid(pos: Point<i32>, grid: &Grid) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.x < grid.len() as i32 && pos.y < grid[0].len() as i32 {
        return true;
    }
    false
}

fn height_diff(grid: &Grid, p1: Point<i32>, p2: Point<i32>) -> i32 {
    grid[p1.x as usize][p1.y as usize] - (grid[p2.x as usize][p2.y as usize]) as i32
}

fn get_neighbors(grid: &Grid, pos: Point<i32>) -> Vec<Point<i32>> {
    let left = pos + Point::new(0, -1);
    let right = pos + Point::new(0, 1);
    let up = pos + Point::new(-1, 0);
    let down = pos + Point::new(1, 0);

    let mut neighbors = Vec::new();

    for neighbor in [left, right, up, down] {
        if is_in_grid(neighbor, grid) && height_diff(grid, neighbor, pos) == 1 {
            neighbors.push(neighbor);
        }
    }

    neighbors
}

fn get_entry_points(grid: &Grid, pos: Point<i32>) -> Vec<Point<i32>> {
    let left = pos + Point::new(0, -1);
    let right = pos + Point::new(0, 1);
    let up = pos + Point::new(-1, 0);
    let down = pos + Point::new(1, 0);

    let mut entry_points = Vec::new();

    for entry_point in [left, right, up, down] {
        if is_in_grid(entry_point, grid) && height_diff(grid, entry_point, pos) == -1 {
            entry_points.push(entry_point);
        }
    }

    entry_points
}

fn get_trailheads(grid: &Grid) -> Vec<Point<i32>> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, v)| {
                if *v == 0 {
                    Some(Point::new(i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn get_num_peaks(grid: &Grid) -> i32 {
    grid.iter()
        .flat_map(|row| {
            row.iter()
                .filter_map(|v| if *v == 9 { Some(1) } else { None })
        })
        .sum()
}

fn depth_first_search(grid: &Grid, trailhead: Point<i32>, num_peaks: i32) -> i32 {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push(trailhead);

    let mut score = 0;
    while let Some(v) = stack.pop() {
        if !visited.contains(&v) {
            visited.insert(v);
            if grid[v.x as usize][v.y as usize] == 9 {
                score += 1;
                if score == num_peaks {
                    return score;
                }
            }
            for neighbor in get_neighbors(grid, v) {
                stack.push(neighbor);
            }
        }
    }

    score
}

fn depth_first_search_part2(grid: &Grid, trailhead: Point<i32>) -> i32 {
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    stack.push(trailhead);

    let mut rating = 0;
    while let Some(v) = stack.pop() {
        if !visited.contains(&v) {
            if get_entry_points(grid, v)
                .iter()
                .all(|entry_point| visited.contains(entry_point))
            {
                visited.insert(v);
            }
            if grid[v.x as usize][v.y as usize] == 9 {
                rating += 1;
            }
            for neighbor in get_neighbors(grid, v) {
                stack.push(neighbor);
            }
        }
    }

    rating
}

fn get_total_score(grid: &Grid) -> i32 {
    let trailheads = get_trailheads(&grid);
    let num_peaks = get_num_peaks(&grid);

    let mut total = 0;

    for trailhead in trailheads {
        // println!("{:?}", trailhead);
        let score = depth_first_search(grid, trailhead, num_peaks);
        // println!("{}", score);
        total += score;
    }

    total
}

fn get_total_rating(grid: &Grid) -> i32 {
    let trailheads = get_trailheads(&grid);

    let mut total = 0;

    for trailhead in trailheads {
        // println!("{:?}", trailhead);
        let score = depth_first_search_part2(grid, trailhead);
        // println!("{}", score);
        total += score;
    }

    total
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let grid: Grid = input
        .lines()
        .map(|line| {
            line.chars()
                // 99, i.e. dots are insurmountable
                .map(|c| c.to_digit(10).unwrap_or(99) as i32)
                .collect()
        })
        .collect();

    // println!("{:?}", grid);

    println!("part 1 {}", get_total_score(&grid));
    println!("part 2 {}", get_total_rating(&grid));
}
