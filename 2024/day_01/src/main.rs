use std::collections::HashMap;
use std::fs;

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let tuples = input.lines().map(|line| {
        let v: Vec<_> = line.split_whitespace().collect();
        (v[0], v[1])
    });

    let mut left: Vec<_> = tuples
        .clone()
        .map(|(left, _)| left.parse::<i32>().unwrap())
        .collect();
    let mut right: Vec<_> = tuples
        .map(|(_, right)| right.parse::<i32>().unwrap())
        .collect();

    left.sort_unstable();
    right.sort_unstable();

    let total_dist: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(left, right)| (right - left).abs())
        .sum();

    println!("total distance = {total_dist}");

    let mut counts = HashMap::new();

    for elem in right {
        counts
            .entry(elem)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let sym_score: i32 = left.iter().map(|v| v * counts.get(v).unwrap_or(&0)).sum();

    println!("symmetry score = {sym_score}");
}
