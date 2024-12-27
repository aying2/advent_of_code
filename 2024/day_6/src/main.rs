use std::ops::{Add, AddAssign, Neg, Sub};
use std::{fs, usize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
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
                    // allow steps which are just turns for counting purposes
                    break self.pos;
                } else {
                    break tmp;
                }
            } else {
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

fn get_test_cases_around_point(pos: &Point, grid: &Vec<Vec<char>>) -> Vec<Guard> {
    let mut ret = Vec::new();

    let tmp = *pos + Point::new(-1, 0);
    if is_in_grid(&tmp, grid) && grid[tmp.x as usize][tmp.y as usize] != '#' {
        ret.push(Guard::new(Direction::Down, tmp));
    }

    let tmp = *pos + Point::new(1, 0);
    if is_in_grid(&tmp, grid) && grid[tmp.x as usize][tmp.y as usize] != '#' {
        ret.push(Guard::new(Direction::Up, tmp));
    }

    let tmp = *pos + Point::new(0, -1);
    if is_in_grid(&tmp, grid) && grid[tmp.x as usize][tmp.y as usize] != '#' {
        ret.push(Guard::new(Direction::Right, tmp));
    }

    let tmp = *pos + Point::new(0, 1);
    if is_in_grid(&tmp, grid) && grid[tmp.x as usize][tmp.y as usize] != '#' {
        ret.push(Guard::new(Direction::Left, tmp));
    }

    ret
}

fn get_test_cases(grid: &Vec<Vec<char>>) -> Vec<Guard> {
    let mut ret = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '#' {
                ret.append(&mut get_test_cases_around_point(
                    &Point::new(i as i32, j as i32),
                    grid,
                ));
            }
        }
    }

    ret
}

fn get_missing_obs_pos(obs_poses: &Vec<Guard>) -> Point {
    assert_eq!(obs_poses.len(), 3);

    let v1_0 = obs_poses[0].pos - obs_poses[1].pos;
    let v1_2 = obs_poses[2].pos - obs_poses[1].pos;

    let mut ret = obs_poses[2].pos + v1_0;

    assert_eq!(ret, obs_poses[0].pos + v1_2);

    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Left,
    ];

    let obs_dirs = obs_poses.iter().map(|g| g.dir).collect::<Vec<_>>();
    let missing_dirs = directions
        .iter()
        .filter(|v| !obs_dirs.contains(v))
        .collect::<Vec<_>>();

    assert_eq!(missing_dirs.len(), 1);

    match missing_dirs[0] {
        Direction::Up => ret += Point::new(-1, 0),
        Direction::Down => ret += Point::new(1, 0),
        Direction::Right => ret += Point::new(0, 1),
        Direction::Left => ret += Point::new(0, -1),
    }

    return ret;
}

fn is_loopable(grid: &Vec<Vec<char>>, guard: &mut Guard) -> bool {
    let grid = &mut grid.clone();

    let mut obs_poses = Vec::new();
    let start = guard.clone();

    let mut prev = guard.clone();

    while is_in_grid(&guard.pos, grid) {
        guard.step(grid);

        if *guard == start {
            return true;
        }

        if guard.dir != prev.dir && obs_poses.len() < 3 {
            obs_poses.push(prev);
            if obs_poses.len() == 3 {
                let missing_obs_pos = get_missing_obs_pos(&obs_poses);
                if !is_in_grid(&missing_obs_pos, grid)
                    || grid[missing_obs_pos.x as usize][missing_obs_pos.y as usize] == '#'
                {
                    return false;
                }
                grid[missing_obs_pos.x as usize][missing_obs_pos.y as usize] = '#';

                println!("{:?}", get_missing_obs_pos(&obs_poses));
            }
        }

        prev = guard.clone();
    }

    false
}

fn simulate_part2(grid: &Vec<Vec<char>>) -> usize {
    let test_cases = get_test_cases(grid);

    let mut total = 0;

    for test_case in test_cases {
        println!("{:?}", test_case);
        let mut guard = test_case.clone();

        if is_loopable(grid, &mut guard) {
            println!("booya");
            total += 1;
        }
    }

    total
}

const INPUT_PATH: &str = "input/example.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut guard = Guard::try_from(&grid).unwrap();

    println!("{}", simulate(&mut grid, &mut guard));

    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    println!("{}", simulate_part2(&grid));
}
