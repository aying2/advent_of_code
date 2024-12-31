use regex::Regex;
use std::fs;
mod point;
use point::*;

#[derive(Debug)]
struct Machine {
    button_a: Point<i64>,
    button_b: Point<i64>,
    prize: Point<i64>,
}

fn n_tokens_to_win(machine: &Machine) -> i64 {
    // X_A * N_A + X_B * N_B = X_P
    // Y_A * N_A + Y_B * N_B = Y_P
    //
    // N_B = (Y_P - Y_A * N_A) / Y_B
    // X_A * N_A + X_B * (Y_P - Y_A * N_A) / Y_B = X_P
    // (X_A - X_B * Y_A / Y_B) * N_A + X_B * Y_P / Y_B = X_P
    // (Y_B * X_A - X_B * Y_A) * N_A  = Y_B * X_P - X_B * Y_P
    // N_A  = (Y_B * X_P - X_B * Y_P) / (Y_B * X_A - X_B * Y_A)

    let n_a = (machine.button_b.y * machine.prize.x - machine.button_b.x * machine.prize.y)
        / (machine.button_b.y * machine.button_a.x - machine.button_b.x * machine.button_a.y);
    let n_b = (machine.prize.x - machine.button_a.x * n_a) / machine.button_b.x;

    // println!("{n_a}, {n_b}");

    let x_p_hat = machine.button_a.x * n_a + machine.button_b.x * n_b;
    let y_p_hat = machine.button_a.y * n_a + machine.button_b.y * n_b;

    if x_p_hat != machine.prize.x || y_p_hat != machine.prize.y {
        return 0;
    }

    return 3 * n_a + n_b;
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let re = Regex::new(r"\d+").unwrap();

    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|block| {
            let machine = block
                .lines()
                .map(|line| {
                    re.find_iter(line)
                        .map(|m| m.as_str().parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            Machine {
                button_a: Point::new(machine[0][0], machine[0][1]),
                button_b: Point::new(machine[1][0], machine[1][1]),
                prize: Point::new(machine[2][0], machine[2][1]),
            }
        })
        .collect();

    // println!("{:?}", machines);

    let n_tokens: i64 = machines
        .iter()
        .map(|machine| n_tokens_to_win(machine))
        .sum();

    println!("part 1: {}", n_tokens);

    const OFFSET: i64 = 10_000_000_000_000;
    const OFFSET_PT: Point<i64> = Point {
        x: OFFSET,
        y: OFFSET,
    };

    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|block| {
            let machine = block
                .lines()
                .map(|line| {
                    re.find_iter(line)
                        .map(|m| m.as_str().parse().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            Machine {
                button_a: Point::new(machine[0][0], machine[0][1]),
                button_b: Point::new(machine[1][0], machine[1][1]),
                prize: Point::new(machine[2][0], machine[2][1]) + OFFSET_PT,
            }
        })
        .collect();

    // println!("{:?}", machines);

    let n_tokens: i64 = machines
        .iter()
        .map(|machine| n_tokens_to_win(machine))
        .sum();

    println!("part 2: {}", n_tokens);
}
