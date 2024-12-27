use regex::Regex;
use std::fs;

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mul_sum: i32 = input
        .lines()
        .flat_map(|hay| {
            let captures = re.captures_iter(hay);
            captures.map(|capture| {
                capture.get(1).unwrap().as_str().parse::<i32>().unwrap()
                    * capture.get(2).unwrap().as_str().parse::<i32>().unwrap()
            })
        })
        .sum();

    println!("part 1: {mul_sum}");

    let window_re = Regex::new(r"(?:^|do\(\)).*?(?:don't\(\)|$)").unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // weird how the directions don't mention you need to persist do's and don't's across lines
    let mul_sum: i32 = window_re
        .find_iter(&input.replace("\n", ""))
        .flat_map(|window| {
            let captures = re.captures_iter(window.as_str());
            captures.map(|capture| {
                let (_, [factor_1, factor_2]) = capture.extract();
                factor_1.parse::<i32>().unwrap() * factor_2.parse::<i32>().unwrap()
            })
        })
        .sum();

    println!("part 2: {mul_sum}")
}
