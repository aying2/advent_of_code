use std::{fs, usize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Block {
    Digit(i64),
    Space,
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

#[derive(Debug)]
struct Run {
    blocks: Vec<Block>,
    blocktype: Block,
}

// fn try_run_slop(space_runs: &mut Vec<Run>, space_idx: usize) -> {
// }
//
// fn try_run_swap(digit_run: &mut Run, space_runs: &mut Vec<Run>, space_idx: usize) -> bool {
//     // TODO: need to merge space with adjacent after pushing to back
//     // fix equal len swap to push to back
//     if space_runs[space_idx].len == digit_run.len {
//         (space_runs[space_idx].idx, digit_run.idx) = (digit_run.idx, space_runs[space_idx].idx);
//
//         // assume the rest of the vector is sorted so only search that partition
//         // then offset the index
//         let new_space_idx = space_runs[(space_idx + 1)..]
//             .binary_search(&space_runs[space_idx])
//             .unwrap_err()
//             + (space_idx + 1);
//
//         space_runs.swap(space_idx, new_space_idx);
//         return true;
//     } else if space_runs[j].len > digit_runs[i].len {
//         // println!("");
//         // println!("{i}, {j}");
//         // println!("{:?}", space_runs);
//         // println!("");
//         // println!("{:?}", digit_runs);
//         space_runs.push(Run {
//             block: Block::Space,
//             len: digit_runs[i].len,
//             idx: digit_runs[i].idx,
//         });
//         (space_runs[j].idx, digit_runs[i].idx) =
//             (space_runs[j].idx + digit_runs[i].len, space_runs[j].idx);
//         space_runs[j].len -= digit_runs[i].len;
//         return true;
//     }
//
//     false
// }
//
// fn order_and_merge_runs(mut digit_runs: Vec<Run>, mut space_runs: Vec<Run>) -> Vec<Run> {
//     println!("{:?}", digit_runs);
//     println!("{:?}", space_runs);
//     for digit_run in digit_runs.iter_mut().rev() {
//         for i in 0..space_runs.len() {
//             // only consider swapping with spaces from indexes earlier than digit
//             if space_runs[i].idx >= digit_run.idx {
//                 break;
//             }
//             if try_run_swap(digit_run, &mut space_runs, i) {
//                 break;
//             }
//
//             // // TODO: need to merge space with adjacent after pushing to back
//             // // fix equal len swap to push to back
//             // if space_runs[j].len == digit_runs[i].len {
//             //     // println!("");
//             //     // println!("{i}, {j}");
//             //     // println!("{:?}", space_runs);
//             //     // println!("");
//             //     // println!("{:?}", digit_runs);
//             //     (space_runs[j].idx, digit_runs[i].idx) = (digit_runs[i].idx, space_runs[j].idx);
//             //     break;
//             // } else if space_runs[j].len > digit_runs[i].len {
//             //     // println!("");
//             //     // println!("{i}, {j}");
//             //     // println!("{:?}", space_runs);
//             //     // println!("");
//             //     // println!("{:?}", digit_runs);
//             //     space_runs.push(Run {
//             //         block: Block::Space,
//             //         len: digit_runs[i].len,
//             //         idx: digit_runs[i].idx,
//             //     });
//             //     (space_runs[j].idx, digit_runs[i].idx) =
//             //         (space_runs[j].idx + digit_runs[i].len, space_runs[j].idx);
//             //     space_runs[j].len -= digit_runs[i].len;
//             //     break;
//             // }
//
//             // if space_runs[j].len >= digit_runs[i].len {
//             //     println!("{i}, {j}");
//             //     println!("{:?}", digit_runs);
//             //     println!("{:?}", space_runs);
//             //     (space_runs[j].idx, digit_runs[i].idx) = (digit_runs[i].idx, space_runs[j].idx);
//             //
//             //     if space_runs[j].len > digit_runs[i].len {
//             //         space_runs[j].len = digit_runs[i].len;
//             //         space_runs.insert(
//             //             j + 1,
//             //             Run {
//             //                 block: Block::Space,
//             //                 len: space_runs[j].len - digit_runs[i].len,
//             //                 idx: space_runs[j].idx + space_runs[j].len,
//             //             },
//             //         )
//             //     }
//             // }
//         }
//     }
//     // println!("{:?}", digit_runs);
//     // println!("{:?}", space_runs);
//     //
//     digit_runs.append(&mut space_runs);
//     digit_runs.sort_unstable();
//     digit_runs
// }
//
//
fn runs_to_blocks(runs: Vec<Run>) -> Vec<Block> {
    runs.into_iter()
        .flat_map(|run| run.blocks.into_iter())
        .collect()
}

// fn slop_runs(runs: &mut Vec<Run>, digit_idx: &mut usize, mut space_idx: usize) {
//     matches!(runs[*digit_idx].blocktype, Block::Space);
//     matches!(runs[space_idx].blocktype, Block::Digit(_));
//     assert!(space_idx < *digit_idx);
//
//     if let Some(idx) = space_idx.checked_sub(1) {
//         if let Block::Digit(_) = runs[idx].blocktype {
//             let (left, right) = runs.split_at_mut(space_idx);
//             left[left.len() - 1].blocks.append(&mut right[0].blocks);
//
//             runs.remove(space_idx);
//             *digit_idx -= 1;
//             space_idx -= 1;
//             println!("{}", digit_idx);
//             println!("{}", space_idx);
//             println!("{:?}", runs);
//         }
//     }
//
//     if let Some(idx) = space_idx.checked_add(1) {
//         if let Block::Digit(_) = runs[idx].blocktype {
//             let (left, right) = runs.split_at_mut(idx);
//             left[left.len() - 1].blocks.append(&mut right[0].blocks);
//
//             runs.remove(space_idx + 1);
//             *digit_idx -= 1;
//
//             println!("LOOOOOOOOOOOOOOOOOOL");
//             println!("{}", digit_idx);
//             println!("{}", space_idx);
//             println!("{:?}", runs);
//         }
//     }
// }
//
// fn reorder_runs(runs: &mut Vec<Run>) {
//     assert!(runs.len() % 2 == 1);
//
//     let mut digit_idx = runs.len() - 1;
//     loop {
//         let digit_len = runs[digit_idx].blocks.len();
//         for space_idx in (1..digit_idx).step_by(2) {
//             let space_len = runs[space_idx].blocks.len();
//             if space_len >= digit_len {
//                 println!("HEY");
//                 println!("{:?}", runs[digit_idx]);
//                 runs.swap(digit_idx, space_idx);
//
//                 if space_len > digit_len {
//                     runs.insert(
//                         space_idx + 1,
//                         Run {
//                             blocks: [Block::Space].repeat(space_len - digit_len),
//                             blocktype: Block::Space,
//                         },
//                     );
//                     digit_idx += 1;
//
//                     println!("{:?}", runs);
//                     println!("{digit_idx}");
//                 }
//
//                 slop_runs(runs, &mut digit_idx, space_idx);
//                 println!("{:?}", runs);
//                 println!("{digit_idx}");
//
//                 break;
//             }
//         }
//
//         // have to construct condition this because of usize overflow
//         if digit_idx < 2 {
//             break;
//         }
//
//         digit_idx -= 2
//     }
// }

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
                    if i % 2 == 0 {
                        return Block::Digit((i / 2) as i64);
                    }
                    Block::Space
                });
            })
        })
        .collect::<Vec<_>>();
    // println!("{:?}", blocks);

    reorder_blocks(&mut blocks);

    // println!("{:?}", blocks);

    println!("part 1: {}", compute_checksum(&blocks));

    let mut runs = input
        .lines()
        .flat_map(|line| {
            line.chars().enumerate().map(|(i, c)| {
                let n = c.to_digit(10).unwrap();
                if i % 2 == 0 {
                    return Run {
                        blocks: [Block::Digit((i / 2) as i64)].repeat(n as usize),
                        blocktype: Block::Digit(-1),
                    };
                }
                Run {
                    blocks: [Block::Space].repeat(n as usize),
                    blocktype: Block::Space,
                }
            })
        })
        .collect::<Vec<_>>();

    // println!("{:?}", runs);

    reorder_runs(&mut runs);

    println!("{:?}", runs);

    let blocks = runs_to_blocks(runs);

    println!("{:?}", blocks);

    println!("part 2: {}", compute_checksum(&blocks));
}
