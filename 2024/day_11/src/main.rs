use std::fs;

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

fn step_stones(mut stones: Vec<i64>) -> Vec<i64> {
    let mut i = 0;
    while i < stones.len() {
        if stones[i] == 0 {
            stones[i] = 1;
        } else if n_digits(stones[i]) % 2 == 0 {
            let split = split_digits(stones[i]);
            stones[i] = split.0;
            stones.insert(i + 1, split.1);
            // skip over inserted stone
            i += 1;
        } else {
            stones[i] *= 2024;
        }

        i += 1;
    }

    stones
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let mut stones = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    // println!("{:?}", stones);

    // TODO naively this takes forever
    // the splitting leads to mostly 0, 2, 4, 8 from the 2024
    // need to track their cycles
    for _ in 0..25 {
        stones = step_stones(stones);
    }

    // println!("{:?}", stones);

    println!("part 1: {}", stones.len());
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
