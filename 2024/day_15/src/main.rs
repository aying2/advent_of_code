use core::panic;
use std::{
    fs::{self, File},
    io::Write,
    mem::swap,
};
mod point;
use point::*;

#[derive(Debug, Clone, Copy)]
enum Tile {
    Robot,
    Box,
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
            step(pos, dir, grid)
        }
        Tile::Wall => pos,
        Tile::Robot => panic!("more than one robot"),
    }
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let tiles: Vec<Robot> = input
        .lines()
        .map(|line| {
            let robot: Vec<_> = re
                .captures_iter(line)
                .map(|cap| {
                    let (_, [pos, vel]) = cap.extract();
                    (pos.parse().unwrap(), vel.parse().unwrap())
                })
                .collect();

            Robot {
                pos: Point::new(robot[0].0, robot[0].1),
                vel: Point::new(robot[1].0, robot[1].1),
            }
        })
        .collect();

    // println!("{:?}", robots);

    let safety_factor = simulate(robots.clone(), 100);

    println!("part 1: {}", safety_factor);

    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(OUTPUT_PATH)
        .unwrap();
    simulate_part2(robots, 10000, &mut file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_point() {
        assert_eq!(wrap_point(&Point::new(10, 10)), Point::new(10, 10));
        assert_eq!(wrap_point(&Point::new(0, 0)), Point::new(0, 0));
        assert_eq!(
            wrap_point(&(GRID_DIMS - Point::new(1, 1))),
            GRID_DIMS - Point::new(1, 1)
        );

        assert_eq!(wrap_point(&GRID_DIMS), Point::new(0, 0));
        assert_eq!(
            wrap_point(&Point::new(-1, -1)),
            GRID_DIMS - Point::new(1, 1)
        );

        assert_eq!(
            wrap_point(&Point::new(4 * GRID_DIMS.x + 4, 5 * GRID_DIMS.y + 5)),
            Point::new(4, 5)
        );
        assert_eq!(
            wrap_point(&Point::new(-4 * GRID_DIMS.x - 4, -5 * GRID_DIMS.y - 5)),
            Point::new(GRID_DIMS.x - 4, GRID_DIMS.y - 5)
        );
    }
}
