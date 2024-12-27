use std::fs;

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
