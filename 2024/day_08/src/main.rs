use std::collections::{HashMap, HashSet};
use std::fs;
mod point;
use point::*;

fn print_grid(grid: &Vec<Vec<char>>) {
    println!(
        "{}",
        grid.iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn compute_antinode_poses(a: Point<i32>, b: Point<i32>) -> (Point<i32>, Point<i32>) {
    (a + (a - b), b + (b - a))
}

fn compute_antinode_poses_part2(
    a: Point<i32>,
    b: Point<i32>,
    grid: &Vec<Vec<char>>,
) -> Vec<Point<i32>> {
    let mut antinodes = Vec::new();

    let mut antinode = a;
    loop {
        if !is_in_grid(antinode, grid) {
            break;
        }
        antinodes.push(antinode);

        antinode += a - b;
    }

    let mut antinode = b;
    loop {
        if !is_in_grid(antinode, grid) {
            break;
        }
        antinodes.push(antinode);

        antinode += b - a;
    }

    antinodes
}

fn is_in_grid(pos: Point<i32>, grid: &Vec<Vec<char>>) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.x < grid.len() as i32 && pos.y < grid[0].len() as i32 {
        return true;
    }
    false
}

fn get_antenna_map(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<Point<i32>>> {
    let mut antennas = HashMap::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c != '.' {
                antennas
                    .entry(*c)
                    .or_insert(Vec::new())
                    .push(Point::new(i as i32, j as i32));
            }
        }
    }
    antennas
}

fn count_valid_antinodes(antennas: &HashMap<char, Vec<Point<i32>>>, grid: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    let mut seen = HashSet::new();
    for (_, antenna_vec) in antennas {
        for i in 0..antenna_vec.len() {
            for j in (i + 1)..antenna_vec.len() {
                let antinodes = compute_antinode_poses(antenna_vec[i], antenna_vec[j]);
                if is_in_grid(antinodes.0, grid) && !seen.contains(&antinodes.0) {
                    seen.insert(antinodes.0);
                    total += 1;
                }
                if is_in_grid(antinodes.1, grid) && !seen.contains(&antinodes.1) {
                    seen.insert(antinodes.1);
                    total += 1;
                }
            }
        }
    }

    total
}

fn count_valid_antinodes_part2(
    antennas: &HashMap<char, Vec<Point<i32>>>,
    grid: &Vec<Vec<char>>,
) -> i32 {
    let mut total = 0;

    let mut seen = HashSet::new();
    for (_, antenna_vec) in antennas {
        for i in 0..antenna_vec.len() {
            for j in (i + 1)..antenna_vec.len() {
                let antinodes = compute_antinode_poses_part2(antenna_vec[i], antenna_vec[j], grid);
                for antinode in antinodes {
                    if !seen.contains(&antinode) {
                        seen.insert(antinode);
                        total += 1;
                    }
                }
            }
        }
    }

    total
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let antennas = get_antenna_map(&grid);

    println!("part 1: {}", count_valid_antinodes(&antennas, &grid));
    println!("part 2: {}", count_valid_antinodes_part2(&antennas, &grid));
}
