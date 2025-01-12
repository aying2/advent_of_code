use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Neg, Sub};
use std::{fs, usize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.add(-other)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Guard {
    dir: Direction,
    pos: Point,
}

impl Guard {
    fn new(dir: Direction, pos: Point) -> Self {
        Self { dir, pos }
    }

    fn try_from(grid: &Vec<Vec<char>>) -> Option<Guard> {
        for (i, line) in grid.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '^' {
                    return Some(Guard {
                        dir: Direction::Up,
                        pos: Point::new(i as i32, j as i32),
                    });
                }
            }
        }

        return None;
    }

    fn marked_step(&mut self, grid: &mut Vec<Vec<char>>) {
        grid[self.pos.x as usize][self.pos.y as usize] = 'X';
        self.step(grid);
    }

    fn marked_step_alt(&mut self, grid: &mut Vec<Vec<char>>) {
        grid[self.pos.x as usize][self.pos.y as usize] = 'O';
        self.step(grid);
    }

    fn step(&mut self, grid: &Vec<Vec<char>>) {
        let planned = loop {
            let mut tmp = self.pos;
            match self.dir {
                Direction::Up => tmp.x -= 1,
                Direction::Down => tmp.x += 1,
                Direction::Left => tmp.y -= 1,
                Direction::Right => tmp.y += 1,
            }
            if is_in_grid(&tmp, grid) {
                if grid[tmp.x as usize][tmp.y as usize] == '#' {
                    match self.dir {
                        Direction::Up => self.dir = Direction::Right,
                        Direction::Down => self.dir = Direction::Left,
                        Direction::Left => self.dir = Direction::Up,
                        Direction::Right => self.dir = Direction::Down,
                    }
                } else {
                    break tmp;
                }
            } else {
                // allow going out of bounds
                break tmp;
            }
        };

        self.pos = planned;
    }
}

fn is_in_grid(pos: &Point, grid: &Vec<Vec<char>>) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.x < grid.len() as i32 && pos.y < grid[0].len() as i32 {
        return true;
    }
    false
}

fn simulate(grid: &mut Vec<Vec<char>>, guard: &mut Guard) -> usize {
    while is_in_grid(&guard.pos, grid) {
        guard.marked_step(grid);
    }

    return grid
        .iter()
        .flat_map(|line| line.iter())
        .filter(|c| **c == 'X')
        .count();
}

fn get_path(grid: &Vec<Vec<char>>, guard: &mut Guard) -> Vec<(Point, Guard)> {
    // NOTE: we do NOT want the first position to be counted in the path here
    // because if we place an obstacle there and we start there
    // the guard stepping won't work properly
    // I ended up getting the right answer by subtracting out this case
    // manually originally
    let mut path = Vec::new();

    let mut visited = HashSet::new();
    // put the first position in visited
    // it's not enought to simply remove it from the path post hoc
    // because the path might intersect with the first position again later on
    visited.insert(guard.pos);
    let mut prev = guard.clone();
    while is_in_grid(&guard.pos, &grid) {
        guard.step(&grid);
        if !is_in_grid(&guard.pos, &grid) {
            break;
        }
        if !visited.contains(&guard.pos) {
            // use HashSet to quickly check uniqueness of positions
            // and vector to keep sequential order
            visited.insert(guard.pos);
            path.push((guard.pos, prev));
        }

        prev = guard.clone();
    }
    path
}

fn is_loop(grid: &Vec<Vec<char>>, guard: &mut Guard, visited_init: &HashSet<Guard>) -> bool {
    let mut visited = visited_init.clone();
    while is_in_grid(&guard.pos, grid) {
        guard.step(grid);

        if visited.contains(guard) {
            return true;
        }
        visited.insert(guard.clone());
    }

    false
}

fn simulate_part2(mut grid: Vec<Vec<char>>, guard: &Guard) -> usize {
    let path = get_path(&grid, &mut guard.clone());
    let mut total = 0;

    let mut visited_init = HashSet::new();
    for (obs_pos, start_guard) in path {
        visited_init.insert(start_guard.clone());
        assert_eq!(grid[obs_pos.x as usize][obs_pos.y as usize], 'X');
        grid[obs_pos.x as usize][obs_pos.y as usize] = '#';
        if is_loop(&grid, &mut start_guard.clone(), &visited_init) {
            total += 1;
        }

        grid[obs_pos.x as usize][obs_pos.y as usize] = 'X';
    }

    total
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!(
        "{}",
        grid.iter()
            .map(|v| v.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    );
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let guard = Guard::try_from(&grid).unwrap();

    println!("{}", simulate(&mut grid, &mut guard.clone()));

    println!("{}", simulate_part2(grid, &guard));
}
