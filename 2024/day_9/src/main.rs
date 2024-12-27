use std::fs;

#[derive(Debug, PartialEq)]
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
}
