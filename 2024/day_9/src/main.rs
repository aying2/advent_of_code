use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Block {
    Digit(i64),
    Space,
}

#[derive(Debug, PartialEq, Eq)]
struct Run {
    block: Block,
    len: usize,
    idx: usize,
}

impl Ord for Run {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl PartialOrd for Run {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.idx.partial_cmp(&other.idx)
    }
}

fn reorder_blocks(blocks: &mut Vec<Block>) {
    let mut front_idx = 0;
    let mut back_idx = blocks.len() - 1;

    'outer: loop {
        while blocks[front_idx] != Block::Space {
            if front_idx >= back_idx {
                break 'outer;
            }
            front_idx += 1;
        }

        loop {
            if front_idx >= back_idx {
                break 'outer;
            }
            match blocks[back_idx] {
                Block::Digit(_) => break,
                _ => (),
            }
            back_idx -= 1;
        }

        blocks.swap(front_idx, back_idx);
    }
}

fn order_and_merge_runs(mut digit_runs: Vec<Run>, mut space_runs: Vec<Run>) -> Vec<Run> {
    println!("{:?}", digit_runs);
    println!("{:?}", space_runs);
    for i in (0..digit_runs.len()).rev() {
        for j in 0..space_runs.len() {
            // TODO: need to merge space with adjacent after pushing to back
            // need to only consider swapping with spaces from indexes earlier than digit
            // use break to early cancel search for this
            // fix equal len swap to push to back
            if space_runs[j].len == digit_runs[i].len {
                // println!("");
                // println!("{i}, {j}");
                // println!("{:?}", space_runs);
                // println!("");
                // println!("{:?}", digit_runs);
                (space_runs[j].idx, digit_runs[i].idx) = (digit_runs[i].idx, space_runs[j].idx);
                break;
            } else if space_runs[j].len > digit_runs[i].len {
                // println!("");
                // println!("{i}, {j}");
                // println!("{:?}", space_runs);
                // println!("");
                // println!("{:?}", digit_runs);
                space_runs.push(Run {
                    block: Block::Space,
                    len: digit_runs[i].len,
                    idx: digit_runs[i].idx,
                });
                (space_runs[j].idx, digit_runs[i].idx) =
                    (space_runs[j].idx + digit_runs[i].len, space_runs[j].idx);
                space_runs[j].len -= digit_runs[i].len;
                break;
            }
            // if space_runs[j].len >= digit_runs[i].len {
            //     println!("{i}, {j}");
            //     println!("{:?}", digit_runs);
            //     println!("{:?}", space_runs);
            //     (space_runs[j].idx, digit_runs[i].idx) = (digit_runs[i].idx, space_runs[j].idx);
            //
            //     if space_runs[j].len > digit_runs[i].len {
            //         space_runs[j].len = digit_runs[i].len;
            //         space_runs.insert(
            //             j + 1,
            //             Run {
            //                 block: Block::Space,
            //                 len: space_runs[j].len - digit_runs[i].len,
            //                 idx: space_runs[j].idx + space_runs[j].len,
            //             },
            //         )
            //     }
            // }
        }
    }
    // println!("{:?}", digit_runs);
    // println!("{:?}", space_runs);
    //
    digit_runs.append(&mut space_runs);
    digit_runs.sort_unstable();
    digit_runs
}

fn runs_to_blocks(runs: &Vec<Run>) -> Vec<Block> {
    runs.iter()
        .flat_map(|run| (0..run.len).map(|_| run.block))
        .collect()
}

fn compute_checksum(blocks: &Vec<Block>) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter_map(|(i, v)| match v {
            Block::Digit(n) => Some(n * (i as i64)),
            Block::Space => None,
        })
        .sum()
}

const INPUT_PATH: &str = "input/example.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let mut blocks = input
        .lines()
        .flat_map(|line| {
            line.chars().enumerate().flat_map(|(i, c)| {
                let n = c.to_digit(10).unwrap();
                return (0..n).map(move |_| {
                    if i % 2 == 1 {
                        return Block::Space;
                    } else {
                        return Block::Digit((i / 2) as i64);
                    }
                });
            })
        })
        .collect::<Vec<_>>();
    // println!("{blocks:?}");

    reorder_blocks(&mut blocks);

    // println!("{blocks:?}");

    println!("part 1: {}", compute_checksum(&blocks));

    // let mut runs = input
    //     .lines()
    //     .flat_map(|line| {
    //         line.chars().enumerate().map(|(i, c)| {
    //             let len = c.to_digit(10).unwrap() as usize;
    //             let block = if i % 2 == 1 {
    //                 Block::Space
    //             } else {
    //                 Block::Digit((i / 2) as i64)
    //             };
    //             Run { block, len }
    //         })
    //     })
    //     .collect::<Vec<_>>();

    let mut space_runs = Vec::new();
    let mut digit_runs = Vec::new();

    let mut idx = 0;
    for line in input.lines() {
        for (i, c) in line.chars().enumerate() {
            let len = c.to_digit(10).unwrap() as usize;
            if i % 2 == 1 {
                space_runs.push(Run {
                    block: Block::Space,
                    len,
                    idx,
                });
            } else {
                digit_runs.push(Run {
                    block: Block::Digit((i / 2) as i64),
                    len,
                    idx,
                })
            };
            idx += len;
        }
    }

    let runs = order_and_merge_runs(digit_runs, space_runs);
    let blocks = runs_to_blocks(&runs);
    println!("{:?}", runs);
    println!("{:?}", blocks);

    println!("part 2: {}", compute_checksum(&blocks));
}
