use std::fs;

const INPUT_PATH: &str = "input/input.txt";

fn is_valid_equation(result: &i64, terms: &[i64], running: i64, depth: usize) -> bool {
    if depth == terms.len() {
        return running == *result;
    }

    return is_valid_equation(result, terms, running + terms[depth], depth + 1)
        || is_valid_equation(result, terms, running * terms[depth], depth + 1);
}

fn is_valid_equation_part_2(result: &i64, terms: &[i64], running: i64, depth: usize) -> bool {
    if depth == terms.len() {
        return running == *result;
    }

    return is_valid_equation_part_2(result, terms, running + terms[depth], depth + 1)
        || is_valid_equation_part_2(result, terms, running * terms[depth], depth + 1)
        || {
            is_valid_equation_part_2(
                result,
                terms,
                (running.to_string() + &terms[depth].to_string())
                    .parse()
                    .unwrap(),
                depth + 1,
            )
        };
}

fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let equations = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(":").unwrap();
            (
                left.parse::<i64>().unwrap(),
                right
                    .trim()
                    .split_whitespace()
                    .map(|e| e.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let total: i64 = equations
        .iter()
        .map(|(result, terms)| {
            if is_valid_equation(result, &terms[1..], terms[0], 0) {
                return *result;
            }
            0
        })
        .sum();

    println!("part 1: {}", total);
    // println!("{:?}", equations);
    let total: i64 = equations
        .iter()
        .map(|(result, terms)| {
            if is_valid_equation_part_2(result, &terms[1..], terms[0], 0) {
                return *result;
            }
            0
        })
        .sum();

    println!("part 2: {}", total);
}
