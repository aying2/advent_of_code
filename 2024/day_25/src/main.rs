use std::fs;

fn is_lock(block: &[&str]) -> bool {
    if block[0] == "#####" && block[block.len() - 1] == "....." {
        return true;
    }

    false
}

fn parse_keys_and_locks(input: &str) -> (Vec<[i32; 5]>, Vec<[i32; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    let blocks = input
        .split("\n\n")
        .map(|s| s.lines().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut buf = [0; 5];
    for block in blocks {
        for line in &block[1..(block.len() - 1)] {
            buf.iter_mut().enumerate().for_each(|(i, v)| {
                if line.chars().nth(i).unwrap() == '#' {
                    *v += 1;
                }
            });
        }

        if is_lock(&block) {
            locks.push(buf.clone());
        } else {
            keys.push(buf.clone());
        }
        buf.fill(0);
    }

    (keys, locks)
}

fn count_fits<const N: usize>(keys: &Vec<[i32; N]>, locks: &Vec<[i32; N]>) -> i32 {
    let mut total = 0;
    for key in keys {
        'outer: for lock in locks {
            for (k, l) in key.iter().zip(lock.iter()) {
                if k + l > 5 {
                    continue 'outer;
                }
            }

            total += 1;
        }
    }

    total
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let (keys, locks) = parse_keys_and_locks(&input);

    println!("part 1: {:?}", count_fits(&keys, &locks));
}
