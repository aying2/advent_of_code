use std::{
    collections::{HashMap, VecDeque},
    fs, usize,
};
mod point;
use point::*;

type Grid = Vec<Vec<char>>;

fn is_in_grid(grid: &Grid, pos: Point<i32>) -> bool {
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
            if is_in_grid(grid, neighbor) && grid[pos.x as usize][pos.y as usize] != '#' {
                return Some(neighbor);
            }
            None
        })
        .collect()
}

fn breadth_first_search(
    grid: &Grid,
    start: Point<i32>,
    end: Point<i32>,
) -> Option<HashMap<Point<i32>, i32>> {
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
                    return Some(dist);
                }

                queue.push_back(neighbor);
            }
        }
    }

    None
}

fn get_start_and_end_points(grid: &Grid) -> (Point<i32>, Point<i32>) {
    let points: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, _)| {
                if grid[i][j] == 'S' || grid[i][j] == 'E' {
                    Some(Point::new(i as i32, j as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    assert_eq!(points.len(), 2);
    assert_ne!(
        grid[points[0].x as usize][points[0].y as usize],
        grid[points[1].x as usize][points[1].y as usize]
    );

    if grid[points[0].x as usize][points[0].y as usize] == 'S' {
        (points[0], points[1])
    } else {
        (points[1], points[0])
    }
}

fn precompute_path(grid: &Grid) -> HashMap<Point<i32>, i32> {
    let (start, end) = get_start_and_end_points(grid);

    breadth_first_search(grid, start, end).unwrap()
}

fn grid_get_point(grid: &Grid, pos: Point<i32>) -> char {
    grid[pos.x as usize][pos.y as usize]
}

fn find_cheat_start_end(grid: &Grid, pos: Point<i32>) -> Option<(Point<i32>, Point<i32>)> {
    if grid_get_point(grid, pos) == '#' {
        let left = pos + Point::new(0, -1);
        let right = pos + Point::new(0, 1);
        let up = pos + Point::new(-1, 0);
        let down = pos + Point::new(1, 0);

        // track shouldn't have forks but make sure
        assert!(![left, right, up, down]
            .into_iter()
            .all(|neighbor| is_in_grid(grid, neighbor) && grid_get_point(grid, neighbor) != '#'));

        if [left, right]
            .into_iter()
            .all(|neighbor| is_in_grid(grid, neighbor) && grid_get_point(grid, neighbor) != '#')
        {
            return Some((left, right));
        }

        if [up, down]
            .into_iter()
            .all(|neighbor| is_in_grid(grid, neighbor) && grid_get_point(grid, neighbor) != '#')
        {
            return Some((up, down));
        }
    }

    None
}

fn find_timesaves(grid: &Grid) -> Vec<i32> {
    let mut timesaves = Vec::new();
    // can precompute path for a significant time save since there are no forks
    let path = precompute_path(grid);

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            // key insight is that the shortcut time is just the difference between the start and
            // end points
            if let Some((start, end)) = find_cheat_start_end(grid, Point::new(i as i32, j as i32)) {
                // subtract two because each cheat takes 2 picoseconds
                timesaves.push(path[&start].abs_diff(path[&end]) as i32 - 2)
            }
        }
    }

    timesaves
}

fn find_timesaves_around_point(
    grid: &Grid,
    start: Point<i32>,
    timesaves: &mut HashMap<[Point<i32>; 2], i32>,
    path: &HashMap<Point<i32>, i32>,
) {
    for i in -20..=20_i32 {
        let hbnd = 20 - i.abs();
        for j in -hbnd..=hbnd {
            let end = start + Point::new(i, j);
            if is_in_grid(grid, end) && grid_get_point(grid, end) != '#' && start != end {
                let mut key = [start, end];
                key.sort_unstable();
                if !timesaves.contains_key(&key) {
                    let dist = i.abs() + j.abs();
                    let timesave = path[&start].abs_diff(path[&end]) as i32 - dist;
                    timesaves.insert(key, timesave);
                }
            }
        }
    }
}

fn find_timesaves_part2(grid: &Grid) -> Vec<i32> {
    let mut timesaves = HashMap::new();
    let path = precompute_path(grid);

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let start = Point::new(i as i32, j as i32);
            if grid_get_point(grid, start) != '#' {
                find_timesaves_around_point(grid, start, &mut timesaves, &path);
            }
        }
    }

    timesaves
        .into_values()
        .filter(|&timesave| timesave > 0)
        .collect()
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    // println!("{:?}", grid);

    println!(
        "part 1: {:?}",
        find_timesaves(&grid)
            .into_iter()
            .filter(|&timesave| timesave >= 100)
            .count()
    );

    println!(
        "part 2: {:?}",
        find_timesaves_part2(&grid)
            .into_iter()
            .filter(|&timesave| timesave >= 100)
            .count()
    );
}
