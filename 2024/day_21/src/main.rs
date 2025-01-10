use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fs, usize,
};
mod point;
use point::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Into,
}

impl Direction {
    fn to_char(&self) -> char {
        match self {
            Self::Up => '^',
            Self::Down => 'v',
            Self::Left => '<',
            Self::Right => '>',
            Self::Into => 'A',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cursor {
    pos: Point<i32>,
    dir: Direction,
}

#[derive(Debug, PartialEq, Eq)]
struct PriorityNode<T> {
    cost: u64,
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

fn is_in_grid<const W: usize, const H: usize>(grid: &[[char; W]; H], pos: Point<i32>) -> bool {
    if pos.x >= 0 && pos.y >= 0 && pos.x < grid.len() as i32 && pos.y < grid[0].len() as i32 {
        return true;
    }
    false
}

fn get_neighbors<const W: usize, const H: usize>(
    grid: &[[char; W]; H],
    pos: Point<i32>,
) -> Vec<Cursor> {
    let left = Cursor {
        pos: pos + Point::new(0, -1),
        dir: Direction::Left,
    };
    let right = Cursor {
        pos: pos + Point::new(0, 1),
        dir: Direction::Right,
    };
    let up = Cursor {
        pos: pos + Point::new(-1, 0),
        dir: Direction::Up,
    };
    let down = Cursor {
        pos: pos + Point::new(1, 0),
        dir: Direction::Down,
    };

    [left, right, up, down]
        .into_iter()
        .filter_map(|neighbor| {
            if is_in_grid(grid, neighbor.pos) && grid_get_point(grid, neighbor.pos) != '#' {
                return Some(neighbor);
            }
            None
        })
        .collect()
}

fn grid_get_point<const W: usize, const H: usize>(grid: &[[char; W]; H], pos: Point<i32>) -> char {
    grid[pos.x as usize][pos.y as usize]
}

fn print_path(prev: &HashMap<(char, char), (char, char)>, tail: (char, char)) {
    let mut cur = &tail;

    while prev.contains_key(cur) {
        println!("{:?}", cur);
        cur = &prev[cur];
    }
    println!("{:?}", cur);
}

struct NumKeypad {
    grid: [[char; 3]; 4],
    master: Box<dyn Master>,
    cost_map: HashMap<(char, char), u64>,
}

impl NumKeypad {
    fn new(master: Box<dyn Master>) -> Self {
        Self {
            grid: [
                ['7', '8', '9'],
                ['4', '5', '6'],
                ['1', '2', '3'],
                ['#', '0', 'A'],
            ],
            master,
            cost_map: HashMap::new(),
        }
    }
    fn compute_code_complexity(&mut self, code: &str) -> Result<u64, &'static str> {
        let mut prev = 'A';
        let seq_len = code
            .chars()
            .map(|c| {
                let ret = self.shortest_path_press(prev, c);
                prev = c;
                ret
            })
            .sum::<Result<u64, _>>()?;
        let numeric = &code[..(code.len() - 1)].parse().unwrap();

        println!("{seq_len}, {numeric}");

        Ok(seq_len * numeric)
    }
}

impl Keypad<3, 4> for NumKeypad {
    fn get_master(&mut self) -> &mut Box<dyn Master> {
        &mut self.master
    }

    fn get_grid(&self) -> &[[char; 3]; 4] {
        &self.grid
    }

    fn get_cost_map(&mut self) -> &mut HashMap<(char, char), u64> {
        &mut self.cost_map
    }

    fn sym_to_pos(sym: char) -> Result<Point<i32>, &'static str> {
        match sym {
            '7' => Ok(Point::new(0, 0)),
            '8' => Ok(Point::new(0, 1)),
            '9' => Ok(Point::new(0, 2)),
            '4' => Ok(Point::new(1, 0)),
            '5' => Ok(Point::new(1, 1)),
            '6' => Ok(Point::new(1, 2)),
            '1' => Ok(Point::new(2, 0)),
            '2' => Ok(Point::new(2, 1)),
            '3' => Ok(Point::new(2, 2)),
            '#' => Ok(Point::new(3, 0)),
            '0' => Ok(Point::new(3, 1)),
            'A' => Ok(Point::new(3, 2)),
            _ => Err("invalid numkeypad symbol"),
        }
    }
}

struct DirKeypad {
    grid: [[char; 3]; 2],
    master: Box<dyn Master>,
    cost_map: HashMap<(char, char), u64>,
}

impl DirKeypad {
    fn new(master: Box<dyn Master>) -> Self {
        Self {
            grid: [['#', '^', 'A'], ['<', 'v', '>']],
            master,
            cost_map: HashMap::new(),
        }
    }
}

impl Keypad<3, 2> for DirKeypad {
    fn get_master(&mut self) -> &mut Box<dyn Master> {
        &mut self.master
    }

    fn get_grid(&self) -> &[[char; 3]; 2] {
        &self.grid
    }

    fn get_cost_map(&mut self) -> &mut HashMap<(char, char), u64> {
        &mut self.cost_map
    }

    fn sym_to_pos(sym: char) -> Result<Point<i32>, &'static str> {
        match sym {
            '#' => Ok(Point::new(0, 0)),
            '^' => Ok(Point::new(0, 1)),
            'A' => Ok(Point::new(0, 2)),
            '<' => Ok(Point::new(1, 0)),
            'v' => Ok(Point::new(1, 1)),
            '>' => Ok(Point::new(1, 2)),
            _ => Err("invalid dirkeypad symbol"),
        }
    }
}

trait Keypad<const W: usize, const H: usize> {
    fn get_grid(&self) -> &[[char; W]; H];
    fn get_master(&mut self) -> &mut Box<dyn Master>;
    fn get_cost_map(&mut self) -> &mut HashMap<(char, char), u64>;
    fn sym_to_pos(sym: char) -> Result<Point<i32>, &'static str>;

    fn shortest_path_press(&mut self, start: char, end: char) -> Result<u64, &'static str> {
        // precomputation speedup
        // unfortunately this leads to mut proliferation though
        if self.get_cost_map().contains_key(&(start, end)) {
            return Ok(self.get_cost_map()[&(start, end)]);
        }
        // we need to keep track of both the keypad's position and its controller because two nodes
        // with the same keypad position but different controller positions arise from different
        // paths and therefore must be different nodes on the graph
        // this was a sneaky bug
        let mut cost = HashMap::new();
        cost.insert((start, 'A'), 0);
        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Reverse(PriorityNode {
            cost: 0,
            data: (start, 'A'),
        }));

        let mut prev = HashMap::new();

        while let Some(node) = priority_queue.pop() {
            let (sym, master_sym) = node.0.data;

            // another sneaky bug
            // we need to consider the cost to "press" the button because we might have two
            // different ways of reaching the end character with the same cost up to that point but
            // different cost for the master to return to 'A'
            // before I was just returning node.0.cost + self.master.get_cost(master_sym, 'A') but
            // this doesn't factor it into the priority node selection
            if sym == '#' {
                // println!("COST {:?}", cost);
                // println!("PREV {:?}", prev);
                // println!("MASTERS_SYM {:?}", master_sym);
                // print_path(&prev, node.0.data);

                self.get_cost_map().insert((start, end), node.0.cost);
                return Ok(node.0.cost);
            }

            // pretend that the end node has a neighbor above it that you must reach to press the
            // button
            // A bit of a silly implementation but it works
            let neighbors = if sym == end {
                vec![Cursor {
                    pos: Self::sym_to_pos('#')?,
                    dir: Direction::Into,
                }]
            } else {
                get_neighbors(&self.get_grid(), Self::sym_to_pos(sym)?)
            };

            for neighbor in neighbors {
                // the key insight here is that the cost to any given point only depends on the state of
                // the keypad and it's direct controller because any controllers further up the
                // chain will have returned to 'A' when the direct controller's key is pressed
                let alt = node.0.cost
                    + self
                        .get_master()
                        .get_cost(master_sym, neighbor.dir.to_char())?;

                let neighbor_sym = grid_get_point(&self.get_grid(), neighbor.pos);

                if !cost.contains_key(&(neighbor_sym, neighbor.dir.to_char()))
                    || alt < cost[&(neighbor_sym, neighbor.dir.to_char())]
                {
                    // println!("SYM {:?}", sym);
                    // println!("NEIGH_SYM {:?}", neighbor_sym);
                    // println!("NEIGH_DIR {:?}", neighbor.dir);
                    // println!("MASTERS_SYM {:?}", master_sym);
                    // println!(
                    //     "score {}",
                    //     self.master
                    //         .shortest_path_press(master_sym, neighbor.dir.to_char())?
                    // );
                    cost.insert((neighbor_sym, neighbor.dir.to_char()), alt);
                    prev.insert((neighbor_sym, neighbor.dir.to_char()), (sym, master_sym));
                    priority_queue.push(Reverse(PriorityNode {
                        cost: alt,
                        data: (neighbor_sym, neighbor.dir.to_char()),
                    }))
                }
            }
        }

        Err("unreachable end")
    }
}

impl Master for DirKeypad {
    fn get_cost(&mut self, start: char, end: char) -> Result<u64, &'static str> {
        self.shortest_path_press(start, end)
    }
}

struct MasterKeypad;

impl Master for MasterKeypad {
    fn get_cost(&mut self, _start: char, _end: char) -> Result<u64, &'static str> {
        Ok(1)
    }
}

trait Master {
    fn get_cost(&mut self, start: char, end: char) -> Result<u64, &'static str>;
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let codes = input.lines().collect::<Vec<_>>();

    let master = MasterKeypad;

    let dirkeypad = DirKeypad::new(Box::new(master));

    let dirkeypad2 = DirKeypad::new(Box::new(dirkeypad));

    let mut numkeypad = NumKeypad::new(Box::new(dirkeypad2));

    println!(
        "{:?}",
        codes
            .iter()
            .map(|code| numkeypad.compute_code_complexity(code).unwrap())
            .sum::<u64>()
    );

    let master = MasterKeypad;

    let mut prev = DirKeypad::new(Box::new(master));

    for _ in 0..24 {
        let dirkeypad = DirKeypad::new(Box::new(prev));
        prev = dirkeypad;
    }

    let mut numkeypad = NumKeypad::new(Box::new(prev));

    println!(
        "{:?}",
        codes
            .iter()
            .map(|code| numkeypad.compute_code_complexity(code).unwrap())
            .sum::<u64>()
    );
}
