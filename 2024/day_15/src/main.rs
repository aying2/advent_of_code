use core::panic;
use std::fs;
mod point;
use point::*;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Robot,
    Box,
    BoxLeft,
    BoxRight,
    Wall,
    Open,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Grid = Vec<Vec<Tile>>;

fn step(pos: Point<i32>, dir: Direction, grid: &mut Grid) -> Point<i32> {
    let planned = match dir {
        Direction::Up => pos + Point::new(-1, 0),
        Direction::Down => pos + Point::new(1, 0),
        Direction::Left => pos + Point::new(0, -1),
        Direction::Right => pos + Point::new(0, 1),
    };

    match grid[planned.x as usize][planned.y as usize] {
        Tile::Open => {
            (
                grid[pos.x as usize][pos.y as usize],
                grid[planned.x as usize][planned.y as usize],
            ) = (
                grid[planned.x as usize][planned.y as usize],
                grid[pos.x as usize][pos.y as usize],
            );
            planned
        }
        Tile::Box => {
            step(planned, dir, grid);

            // prevent infinite recursion
            // stop if box remains there
            if matches!(grid[planned.x as usize][planned.y as usize], Tile::Open) {
                // i.e. the swap step
                step(pos, dir, grid)
            } else {
                pos
            }
        }
        Tile::BoxLeft | Tile::BoxRight => {
            if matches!(dir, Direction::Left | Direction::Right) {
                step(planned, dir, grid);

                // prevent infinite recursion
                // stop if box remains there
                if matches!(grid[planned.x as usize][planned.y as usize], Tile::Open) {
                    // i.e. the swap step
                    step(pos, dir, grid)
                } else {
                    pos
                }
            } else {
                let planned_adj =
                    if matches!(grid[planned.x as usize][planned.y as usize], Tile::BoxLeft) {
                        planned + Point::new(0, 1)
                    } else {
                        planned + Point::new(0, -1)
                    };

                // make a copy
                // because we only want to shift the grid around
                // if both Box halves can move
                let mut grid_copy = grid.clone();
                step(planned, dir, &mut grid_copy);
                step(planned_adj, dir, &mut grid_copy);

                // prevent infinite recursion
                // stop if box remains there
                if matches!(
                    (
                        grid_copy[planned.x as usize][planned.y as usize],
                        grid_copy[planned_adj.x as usize][planned_adj.y as usize]
                    ),
                    (Tile::Open, Tile::Open)
                ) {
                    *grid = grid_copy;
                    // i.e. the swap step
                    step(pos, dir, grid)
                } else {
                    pos
                }
            }
        }
        Tile::Wall => pos,
        Tile::Robot => panic!("more than one robot"),
    }
}

fn get_robot_pos(grid: &Grid) -> Point<i32> {
    for (i, row) in grid.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if matches!(c, Tile::Robot) {
                return Point::new(i as i32, j as i32);
            }
        }
    }

    panic!("no robot in grid");
}

fn get_box_poses(grid: &Grid) -> Vec<Point<i32>> {
    grid.iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter().enumerate().filter_map(move |(j, c)| {
                // for big box, use closest edge to left side
                if matches!(c, Tile::Box | Tile::BoxLeft) {
                    return Some(Point::new(i as i32, j as i32));
                }
                None
            })
        })
        .collect()
}

fn compute_gps_coord_sum(grid: &Grid) -> i32 {
    get_box_poses(grid)
        .iter()
        .fold(0, |acc, e| acc + 100 * e.x + e.y)
}

fn print_grid(grid: &Grid) {
    println!(
        "{}",
        grid.iter()
            .map(|row| row
                .iter()
                .map(|tile| match tile {
                    Tile::Robot => "@",
                    Tile::Wall => "#",
                    Tile::Open => ".",
                    Tile::Box => "O",
                    Tile::BoxLeft => "[",
                    Tile::BoxRight => "]",
                })
                .collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn simulate(grid: &mut Grid, directions: Vec<Direction>) {
    let mut robot = get_robot_pos(grid);

    // println!("{:?}", robot);
    // print_grid(grid);

    for direction in directions {
        // println!("{:?}", direction);
        robot = step(robot, direction, grid);
        // println!("{:?}", robot);
        // print_grid(grid);
    }
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let split: Vec<_> = input.split("\n\n").collect();

    let mut grid: Grid = split[0]
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '.' => Tile::Open,
                    '@' => Tile::Robot,
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect();

    let directions: Vec<_> = split[1]
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '>' => Direction::Right,
                '<' => Direction::Left,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => panic!("invalid char"),
            })
        })
        .collect();

    // println!("{:?}", grid);
    // println!("{:?}", directions);

    simulate(&mut grid, directions.clone());
    println!("part 1: {}", compute_gps_coord_sum(&grid));

    let mut grid: Grid = split[0]
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| match c {
                    '#' => [Tile::Wall, Tile::Wall],
                    'O' => [Tile::BoxLeft, Tile::BoxRight],
                    '.' => [Tile::Open, Tile::Open],
                    '@' => [Tile::Robot, Tile::Open],
                    _ => panic!("invalid char"),
                })
                .collect()
        })
        .collect();

    // print_grid(&grid);

    simulate(&mut grid, directions);
    println!("part 2: {}", compute_gps_coord_sum(&grid));
}
