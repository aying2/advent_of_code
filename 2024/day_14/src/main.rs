use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
};
mod point;
use point::*;

const GRID_DIMS: Point<i32> = Point { x: 101, y: 103 };
// const GRID_DIMS: Point<i32> = Point { x: 11, y: 7 }; // example

fn print_grid(robots: &Vec<Robot>) {
    let mut grid = [[0; GRID_DIMS.x as usize]; GRID_DIMS.y as usize];

    for robot in robots {
        grid[robot.pos.y as usize][robot.pos.x as usize] += 1;
    }

    println!(
        "{}",
        grid.iter()
            .map(|row| row
                .iter()
                .map(|n| {
                    if *n == 0 {
                        return ".".to_string();
                    }
                    n.to_string()
                })
                .collect::<String>()
                + "\n")
            .collect::<String>()
    )
}

fn write_grid(robots: &Vec<Robot>, file: &mut File, i: i32) {
    let mut grid = [[0; GRID_DIMS.x as usize]; GRID_DIMS.y as usize];

    for robot in robots {
        grid[robot.pos.y as usize][robot.pos.x as usize] += 1;
    }

    writeln!(
        file,
        "{i}\n{}",
        grid.iter()
            .map(|row| row
                .iter()
                .map(|n| {
                    if *n == 0 {
                        return ".".to_string();
                    }
                    n.to_string()
                })
                .collect::<String>()
                + "\n")
            .collect::<String>()
    )
    .unwrap();
}

fn wrap_point(pos: &Point<i32>) -> Point<i32> {
    Point::new(
        if pos.x < 0 {
            GRID_DIMS.x + (pos.x % GRID_DIMS.x)
        } else {
            pos.x % GRID_DIMS.x
        },
        if pos.y < 0 {
            GRID_DIMS.y + (pos.y % GRID_DIMS.y)
        } else {
            pos.y % GRID_DIMS.y
        },
    )
}

#[derive(Debug, Clone)]
struct Robot {
    pos: Point<i32>,
    vel: Point<i32>,
}

impl Robot {
    fn step(&mut self) {
        self.pos = wrap_point(&(self.pos + self.vel));
    }
}

fn get_quadrant_counts(robots: &Vec<Robot>) -> [i32; 4] {
    let mut quadrant_counts = [0; 4];
    for robot in robots {
        // don't count midpoints
        // assume wrapped, i.e. within bounds
        // quadrant definitions don't really matter per se
        if robot.pos.x < (GRID_DIMS.x / 2) {
            if robot.pos.y < (GRID_DIMS.y / 2) {
                quadrant_counts[1] += 1;
            } else if robot.pos.y > (GRID_DIMS.y / 2) {
                quadrant_counts[2] += 1;
            }
        } else if robot.pos.x > (GRID_DIMS.x / 2) {
            if robot.pos.y < (GRID_DIMS.y / 2) {
                quadrant_counts[0] += 1;
            } else if robot.pos.y > (GRID_DIMS.y / 2) {
                quadrant_counts[3] += 1;
            }
        }
    }

    quadrant_counts
}

fn simulate(mut robots: Vec<Robot>, n_steps: u32) -> i32 {
    for _ in 0..n_steps {
        // println!("{i}");
        // println!("{:?}", robots);
        for robot in robots.iter_mut() {
            robot.step();
        }
    }

    // println!("{:?}", get_quadrant_counts(robots));
    get_quadrant_counts(&robots)
        .into_iter()
        .reduce(|acc, e| acc * e)
        .unwrap()
}

fn simulate_part2(mut robots: Vec<Robot>, n_steps: i32, file: &mut File) {
    for i in 0..n_steps {
        // found by inspection
        // unsuprisingly the period of the tree depends on the wrap bounds
        if (i - 39) % 101 == 0 {
            write_grid(&robots, file, i);
        }

        for robot in robots.iter_mut() {
            robot.step();
        }
    }
}

const INPUT_PATH: &str = "input/input.txt";
const OUTPUT_PATH: &str = "output/output.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let re = Regex::new(r"(-?\d+),(-?\d+)").unwrap();

    let robots: Vec<Robot> = input
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
