use std::{collections::HashMap, fs};

fn n_digits(x: i64) -> i64 {
    let mut x = x.abs();
    let mut n = 1;
    while x >= 10 {
        x /= 10;
        n += 1;
    }

    n
}

fn split_digits(x: i64) -> (i64, i64) {
    let n = n_digits(x) as u32;
    let denom = 10_i64.pow(n / 2);
    let left = x / denom;
    let right = x % denom;

    (left, right)
}

// fn step_stones_naive(mut stones: Vec<i64>) -> Vec<i64> {
//     let mut i = 0;
//     while i < stones.len() {
//         if stones[i] == 0 {
//             stones[i] = 1;
//         } else if n_digits(stones[i]) % 2 == 0 {
//             let split = split_digits(stones[i]);
//             stones[i] = split.0;
//             stones.insert(i + 1, split.1);
//             // skip over inserted stone
//             i += 1;
//         } else {
//             stones[i] *= 2024;
//         }
//
//         i += 1;
//     }
//
//     stones
// }

fn step_stones(stones: HashMap<i64, i64>) -> HashMap<i64, i64> {
    let mut ret = HashMap::new();

    for (stone, count) in stones {
        if stone == 0 {
            ret.entry(1).and_modify(|e| *e += count).or_insert(count);
        } else if n_digits(stone) % 2 == 0 {
            let split = split_digits(stone);
            ret.entry(split.0)
                .and_modify(|e| *e += count)
                .or_insert(count);
            ret.entry(split.1)
                .and_modify(|e| *e += count)
                .or_insert(count);
        } else {
            ret.entry(stone * 2024)
                .and_modify(|e| *e += count)
                .or_insert(count);
        }
    }

    ret
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    // let stones_naive = input
    //     .split_whitespace()
    //     .map(|s| s.parse::<i64>().unwrap())
    //     .collect::<Vec<_>>();

    let mut stones = HashMap::new();

    for s in input.split_whitespace() {
        *stones.entry(s.parse::<i64>().unwrap()).or_insert(0) += 1;
    }

    // println!("{:?}", stones);

    for _ in 0..25 {
        stones = step_stones(stones);
        // println!("{:?}", stones);
    }

    println!("part 1: {}", stones.values().sum::<i64>());
    let mut stones = HashMap::new();

    for s in input.split_whitespace() {
        *stones.entry(s.parse::<i64>().unwrap()).or_insert(0) += 1;
    }

    // println!("{:?}", stones);

    for _ in 0..75 {
        stones = step_stones(stones);
        // println!("{:?}", stones);
    }

    println!("part 2: {}", stones.values().sum::<i64>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n_digits() {
        assert_eq!(n_digits(10), 2);
        assert_eq!(n_digits(0), 1);
        assert_eq!(n_digits(1), 1);
        assert_eq!(n_digits(192), 3);
        assert_eq!(n_digits(-192), 3);
    }

    #[test]
    fn test_split_digits() {
        assert_eq!(split_digits(10), (1, 0));
        assert_eq!(split_digits(100), (10, 0));
        assert_eq!(split_digits(1000), (10, 0));
        assert_eq!(split_digits(1234), (12, 34));
    }
}
