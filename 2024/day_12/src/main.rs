use std::{
    collections::{HashSet, VecDeque},
    fs, usize,
};
mod point;
use point::*;

type Grid = Vec<Vec<char>>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn is_in_grid(pos: Point<i32>, grid: &Grid) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.x < grid.len() as i32 && pos.y < grid[0].len() as i32 {
        return true;
    }
    false
}

fn label_eq(grid: &Grid, p1: Point<i32>, p2: Point<i32>) -> bool {
    grid[p1.x as usize][p1.y as usize] == (grid[p2.x as usize][p2.y as usize])
}

fn is_valid_neighbor(grid: &Grid, p1: Point<i32>, p2: Point<i32>) -> bool {
    is_in_grid(p1, grid) && label_eq(grid, p1, p2)
}

fn get_neighbors(grid: &Grid, pos: Point<i32>) -> Vec<Point<i32>> {
    let left = pos + Point::new(0, -1);
    let right = pos + Point::new(0, 1);
    let up = pos + Point::new(-1, 0);
    let down = pos + Point::new(1, 0);

    [left, right, up, down]
        .into_iter()
        .filter_map(|neighbor| {
            if is_valid_neighbor(grid, neighbor, pos) {
                return Some(neighbor);
            }
            None
        })
        .collect()
}

// fn get_8_connectivity(grid: &Grid, pos: Point<i32>) -> [[bool; 3]; 3] {
//     let left = pos + Point::new(0, -1);
//     let right = pos + Point::new(0, 1);
//     let up = pos + Point::new(-1, 0);
//     let down = pos + Point::new(1, 0);
//
//     let up_left = pos + Point::new(-1, -1);
//     let up_right = pos + Point::new(-1, 1);
//     let down_left = pos + Point::new(1, -1);
//     let down_right = pos + Point::new(1, 1);
//
//     [
//         [up_left, up, up_right],
//         [left, pos, right],
//         [down_left, down, down_right],
//     ]
//     .map(|row| row.map(|neighbor| is_in_grid(neighbor, grid) && label_eq(grid, neighbor, pos)))
// }

fn get_grid_points(grid: &Grid) -> HashSet<Point<i32>> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(move |(j, _)| Point::new(i as i32, j as i32))
        })
        .collect()
}

fn depth_first_search(
    grid: &Grid,
    start: Point<i32>,
    not_visited: &mut HashSet<Point<i32>>,
) -> i32 {
    let mut stack = Vec::new();
    stack.push(start);

    let mut perimeter = 0;
    let mut area = 0;
    while let Some(v) = stack.pop() {
        if not_visited.contains(&v) {
            not_visited.remove(&v);
            area += 1;

            let neighbors = get_neighbors(grid, v);
            perimeter += 4 - neighbors.len();
            for neighbor in neighbors {
                stack.push(neighbor);
            }
        }
    }

    (area * perimeter) as i32
}

// fn compute_nsides(pos: Point<i32>, grid: &Grid, not_visited: &HashSet<Point<i32>>) -> i32 {
//     let neighbors = get_neighbors(grid, pos);
//     let gaps = get_gaps(grid, pos);
//     let mut total = gaps.len() as i32;
//
//     'outer: for gap in gaps {
//         for &neighbor in &neighbors {
//             // println!("neighbor {:?}", neighbor);
//             if not_visited.contains(&neighbor) {
//                 // println!("not visit {:?}", not_visited.contains(&neighbor));
//                 continue;
//             }
//             let neighbor_gaps = get_gaps(grid, neighbor);
//             for neighbor_gap in neighbor_gaps {
//                 let diff = neighbor_gap - gap;
//                 let man_dist = diff.x.abs() + diff.y.abs();
//                 if man_dist == 1 {
//                     total -= 1;
//                     continue 'outer;
//                 }
//             }
//         }
//     }
//
//     total
// }

fn count_corners(grid: &Grid, pos: Point<i32>) -> i32 {
    let left = pos + Point::new(0, -1);
    let right = pos + Point::new(0, 1);
    let up = pos + Point::new(-1, 0);
    let down = pos + Point::new(1, 0);

    let mut count = 0;

    for neighbor_a in [left, right] {
        for neighbor_b in [up, down] {
            // outer corners
            if !is_valid_neighbor(grid, neighbor_a, pos)
                && !is_valid_neighbor(grid, neighbor_b, pos)
            {
                count += 1;
            }

            // subtract out extra pos
            let intermediate_neighbor = neighbor_a + neighbor_b - pos;

            // inner corners
            if !is_valid_neighbor(grid, intermediate_neighbor, pos)
                && is_valid_neighbor(grid, neighbor_a, pos)
                && is_valid_neighbor(grid, neighbor_b, pos)
            {
                count += 1;
            }
        }
    }

    count
}
fn depth_first_search_part2(
    grid: &Grid,
    start: Point<i32>,
    not_visited: &mut HashSet<Point<i32>>,
) -> i32 {
    let mut stack = Vec::new();
    stack.push(start);

    let mut nsides = 0;
    let mut area = 0;
    while let Some(v) = stack.pop() {
        if not_visited.contains(&v) {
            not_visited.remove(&v);
            area += 1;

            // println!("{:?}", v);

            // the key insight is that nsides = nturns = ncorners
            let ncorners = count_corners(grid, v);
            // println!("{}", ncorners);
            nsides += ncorners;

            let neighbors = get_neighbors(grid, v);
            for neighbor in neighbors {
                stack.push(neighbor);
            }
        }
    }
    // println!("nsides {}", nsides);

    area * nsides
}

fn get_total_price(grid: &Grid) -> i32 {
    let mut not_visited = get_grid_points(&grid);

    let mut total = 0;

    while !not_visited.is_empty() {
        let start = not_visited.iter().next().unwrap();
        // println!("{:?}", grid[start.x as usize][start.y as usize]);
        let price = depth_first_search(grid, *start, &mut not_visited);
        // println!("{}", price);
        total += price;
    }

    total
}

fn get_total_price_part2(grid: &Grid) -> i32 {
    let mut not_visited = get_grid_points(&grid);

    let mut total = 0;

    while !not_visited.is_empty() {
        let start = not_visited.iter().next().unwrap();
        // println!("{:?}", grid[start.x as usize][start.y as usize]);
        let price = depth_first_search_part2(grid, *start, &mut not_visited);
        // println!("{}", price);
        total += price;
    }

    total
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    // println!("{:?}", grid);

    println!("part 1 {}", get_total_price(&grid));
    println!("part 2 {}", get_total_price_part2(&grid));
}
