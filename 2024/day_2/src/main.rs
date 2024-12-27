use std::fs;

const INPUT_PATH: &str = "input/input.txt";

fn is_safe(report: &Vec<i32>) -> bool {
    let mut prev = report[0];

    let mut prev_diff = 0;

    for &elem in &report[1..] {
        let diff = elem - prev;
        if diff.abs() < 1 || diff.abs() > 3 || (prev_diff != 0 && prev_diff * diff < 0) {
            return false;
        }

        prev = elem;
        prev_diff = diff;
    }

    true
}

// there's probably a smarter way of doing this
// like checking 2 levels behind or searching a tree
// but it's hard since you might not know which level to remove
// until after examining them all, i.e. if the level 1->2 is increasing and
// all the rest are decreasing
fn is_safe_tolerate(report: &Vec<i32>) -> bool {
    for i in 0..report.len() {
        let (left, right) = report.split_at(i);

        let merged = [left, &right[1..]].concat();

        if is_safe(&merged) {
            return true;
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let num_safe: i32 = input
        .lines()
        .map(|line| {
            let v: Vec<_> = line
                .split_whitespace()
                .map(|elem| elem.parse::<i32>().unwrap())
                .collect();
            is_safe(&v) as i32
        })
        .sum();

    println!("part 1: {num_safe}");

    let num_safe: i32 = input
        .lines()
        .map(|line| {
            let v: Vec<_> = line
                .split_whitespace()
                .map(|elem| elem.parse::<i32>().unwrap())
                .collect();
            is_safe_tolerate(&v) as i32
        })
        .sum();

    println!("part 2: {num_safe}");
}
