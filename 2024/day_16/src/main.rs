use core::panic;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fs, usize,
};
mod point;
use point::*;

type Grid = Vec<Vec<char>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cursor {
    pos: Point<i32>,
    dir: Direction,
}

#[derive(Debug, PartialEq, Eq)]
struct PriorityNode<T> {
    cost: u32,
    data: T,
}

impl<T> Ord for PriorityNode<T>
where
    T: Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl<T> PartialOrd for PriorityNode<T>
where
    T: PartialEq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

fn is_in_grid(pos: Point<i32>, grid: &Grid) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.x < grid.len() as i32 && pos.y < grid[0].len() as i32 {
        return true;
    }
    false
}

fn get_opposite_direction(dir: Direction) -> Direction {
    match dir {
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    }
}

fn get_neighbors(grid: &Grid, curs: Cursor) -> Vec<Cursor> {
    let left = Cursor {
        pos: curs.pos + Point::new(0, -1),
        dir: Direction::Left,
    };
    let right = Cursor {
        pos: curs.pos + Point::new(0, 1),
        dir: Direction::Right,
    };
    let up = Cursor {
        pos: curs.pos + Point::new(-1, 0),
        dir: Direction::Up,
    };
    let down = Cursor {
        pos: curs.pos + Point::new(1, 0),
        dir: Direction::Down,
    };

    [left, right, up, down]
        .into_iter()
        .filter_map(|neighbor| {
            if neighbor.dir != get_opposite_direction(neighbor.dir)
                && is_in_grid(neighbor.pos, grid)
                && grid[neighbor.pos.x as usize][neighbor.pos.y as usize] != '#'
            {
                return Some(neighbor);
            }
            None
        })
        .collect()
}

fn get_cost_diff(p1: Cursor, p2: Cursor) -> u32 {
    (if p1.pos == p2.pos { 0 } else { 1 }) + (if p1.dir == p2.dir { 0 } else { 1000 })
}

fn get_start_and_end_points(grid: &Grid) -> (Cursor, Point<i32>) {
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
        (
            Cursor {
                pos: points[0],
                dir: Direction::Right,
            },
            points[1],
        )
    } else {
        (
            Cursor {
                pos: points[1],
                dir: Direction::Right,
            },
            points[0],
        )
    }
}

fn shortest_path_search(grid: &Grid, start: Cursor, end: Point<i32>) -> u32 {
    // use cursors for cost and priority_queue
    // because two cells with the same pos but different dir are not the same
    let mut cost = HashMap::new();
    cost.insert(start, 0);
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(PriorityNode {
        cost: 0,
        data: start,
    }));

    while let Some(node) = priority_queue.pop() {
        if node.0.data.pos == end {
            return node.0.cost;
        }
        for neighbor in get_neighbors(grid, node.0.data) {
            let alt = node.0.cost + get_cost_diff(node.0.data, neighbor);

            if !cost.contains_key(&neighbor) || alt < cost[&neighbor] {
                cost.entry(neighbor).and_modify(|e| *e = alt).or_insert(alt);
                priority_queue.push(Reverse(PriorityNode {
                    cost: alt,
                    data: neighbor,
                }))
            }
        }
    }

    panic!("unreachable end");
}

fn shortest_path_search_part2(grid: &Grid, start: Cursor, end: Point<i32>) -> u32 {
    let mut cost = HashMap::new();
    cost.insert(start, 0);

    let mut prev = HashMap::new();

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse(PriorityNode {
        cost: 0,
        data: start,
    }));

    while let Some(node) = priority_queue.pop() {
        for neighbor in get_neighbors(grid, node.0.data) {
            let alt_cost = node.0.cost + get_cost_diff(node.0.data, neighbor);

            // change < to <= to consider all shortest paths
            if !cost.contains_key(&neighbor) || alt_cost < cost[&neighbor] {
                cost.entry(neighbor)
                    .and_modify(|e| *e = alt_cost)
                    .or_insert(alt_cost);

                // wipe existing entries because they have/share a cost
                // which is now undercut
                // can use Vec instead of Set because each node is only visited once
                // still use cursor instead of pos because we don't want to wipe
                // a shared pos but different position because the cost
                // may not be lower for the other case
                prev.entry(neighbor)
                    .and_modify(|e: &mut Vec<_>| e.clear())
                    .or_insert(Vec::new())
                    .push(node.0.data);

                priority_queue.push(Reverse(PriorityNode {
                    cost: alt_cost,
                    data: neighbor,
                }))
            // should be safe even with lazy eval because contains check is first
            } else if alt_cost == cost[&neighbor] {
                // no need to revisit the node again though because we already
                // added its neighbors for examination
                prev.get_mut(&neighbor).unwrap().push(node.0.data);
            }
        }
    }

    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    // pretty bad, but need to handle ties approaching end from multiple directions
    let mut min_cost = u32::MAX;
    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        let tmp = Cursor { pos: end, dir };
        if cost.contains_key(&tmp) && cost[&tmp] < min_cost {
            min_cost = cost[&tmp];
        }
    }

    for dir in [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ] {
        let tmp = Cursor { pos: end, dir };
        if cost.contains_key(&tmp) && cost[&tmp] == min_cost {
            stack.push(tmp);
        }
    }

    while let Some(cur) = stack.pop() {
        visited.insert(cur.pos);
        if cur != start {
            stack.extend(prev[&cur].clone());
        }
    }

    visited.len() as u32
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
                        if visited.contains(&Point::new(i as i32, j as i32)) {
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

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let grid: Grid = input.lines().map(|line| line.chars().collect()).collect();

    // println!("{:?}", grid);

    let (start, end) = get_start_and_end_points(&grid);

    let min_cost = shortest_path_search(&grid, start, end);

    println!("part 1: {}", min_cost);

    let nseats = shortest_path_search_part2(&grid, start, end);

    println!("part 2: {}", nseats);
}
