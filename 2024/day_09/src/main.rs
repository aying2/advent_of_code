use std::{cmp::Ordering, fs};

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
    for digit_run in digit_runs.iter_mut().rev() {
        for space_idx in 0..space_runs.len() {
            // only consider swapping with spaces from indexes earlier than digit
            if space_runs[space_idx].idx >= digit_run.idx {
                break;
            }

            // i.e. they fit perfectly
            if space_runs[space_idx].len == digit_run.len {
                (space_runs[space_idx].idx, digit_run.idx) =
                    (digit_run.idx, space_runs[space_idx].idx);

                let removed = space_runs.remove(space_idx);
                space_runs.push(removed);
                break;
            } else if space_runs[space_idx].len > digit_run.len {
                // swap the order of operations for simplicity (no removals)
                // push back the "swapped" segment of space_run
                // we don't really care about it's position in the Vec so long as it's out of the
                // way
                space_runs.push(Run {
                    block: Block::Space,
                    len: digit_run.len,
                    idx: digit_run.idx,
                });

                // actually change space_run to be the "residual"
                // i.e. use the fact that it can be in the same location in the Vec
                // swap digit run as normal
                (space_runs[space_idx].idx, digit_run.idx) = (
                    space_runs[space_idx].idx + digit_run.len,
                    space_runs[space_idx].idx,
                );
                space_runs[space_idx].len -= digit_run.len;

                break;
            }
        }
    }
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

const INPUT_PATH: &str = "input/input.txt";
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

    // this took forever to get working and I tried a bunch of different approaches
    //
    // one idea was to use next and prev pointers instead of index to keep track of the separate
    // lists, but 1) pointers in Rust are a travesty that I will deal with later and 2) I figured
    // having to derefence Vecs of pointers would kind of be slow--not linked list slow--but
    // somewhat defeating the point
    //
    // the main idea of this approach is that we can split the runs into digit runs and space runs
    // where the digit runs are iterated back to front and for each iteration we search the space
    // runs front to back until we hit a space run which can fit the digit run.
    // to swap them, we can just swap the "merged indices", i.e. what their positions in the
    // combined Vec with digit and space runs would be.
    // we only need to take the space run out and shove it to the back then so it doesn't mess with
    // the vector's internal order (we know that its merged indices will be behind any subsequent
    // digit runs so it won't come into play with the search).
    // another advantage of this approach over one where the digit and space runs are in the same
    // vector is that our iterations are not interrupted by any mutations we do the Vecs (the digit
    // run Vec's size doesn't change and the space run Vec grows from the back)
    //
    // I also tried an approach where you keep the vectors together and try to quickly jump to the
    // next digit or space run by using a stride of 2, but this requires a lot of maintenance
    // joining adjacent runs together and offsetting the digit run cursor after insertions and
    // merges was a massive headache
    let runs = order_and_merge_runs(digit_runs, space_runs);
    let blocks = runs_to_blocks(&runs);
    // println!("{:?}", runs);
    // println!("{:?}", blocks);

    println!("part 2: {}", compute_checksum(&blocks));
}
