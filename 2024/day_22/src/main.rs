use std::collections::{HashMap, VecDeque};
use std::fs;

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(a: i64) -> i64 {
    a % 16777216
}

fn step_secret(mut secret: i64) -> i64 {
    secret = prune(mix(secret * 64, secret));
    secret = prune(mix(secret / 32, secret));
    secret = prune(mix(secret * 2048, secret));

    secret
}

fn ones_digit(x: i64) -> i64 {
    x % 10
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let secrets = input
        .lines()
        .map(|line| {
            let mut secret = line.parse::<i64>().unwrap();

            for _ in 0..2000 {
                secret = step_secret(secret);
            }
            secret
        })
        .collect::<Vec<_>>();

    println!("part 1: {:?}", secrets.iter().sum::<i64>());

    let mut diffseqs_freqs = HashMap::new();

    // NOTE: faster to use fixed-size array than VecDeque
    // could probably make this faster using a single HashMap for all starting secrets and
    // keeping track of whether a sequence has been seen for a starting secret
    // in a HashMap of HashSets where the keys are sequences and the Sets contain the starting
    // secret index
    // then freqs would instead take the length of this Set
    // could also keep track of see secret sequences like a river fed into by multiple streams
    // but not sure if different seeds coalesce to the same sequence
    let diffseq_prices_vec = input
        .lines()
        .map(|line| {
            let mut diffseq_prices = HashMap::new();
            let mut secret = line.parse::<i64>().unwrap();
            let mut ones_prev = ones_digit(secret);

            let mut diffs = [0; 4];
            for i in 0..2000 {
                secret = step_secret(secret);
                let ones = ones_digit(secret);
                diffs.rotate_left(1);
                diffs[3] = ones - ones_prev;
                if i >= 3 {
                    *diffseqs_freqs.entry(diffs).or_insert(0) += 1;
                    diffseq_prices.entry(diffs).or_insert(ones);
                }

                ones_prev = ones;
            }
            diffseq_prices
        })
        .collect::<Vec<_>>();
    println!("built hashmaps");

    // lower bound makes things a lot faster
    let mut best_price = 0;
    for (diffseq, freq) in &diffseqs_freqs {
        if freq * 9 > best_price {
            let price = diffseq_prices_vec
                .iter()
                .map(|diffseq_prices| diffseq_prices.get(diffseq).unwrap_or(&0))
                .sum::<i64>();

            best_price = best_price.max(price);
        }
    }

    println!("part 2: {:?}", best_price);
}
