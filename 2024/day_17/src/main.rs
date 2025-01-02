use std::fs;

fn adv(registers: &mut Vec<i64>, operand: i64) {
    registers[0] /= 2_i64.pow(combo_operand(registers, operand) as u32);
}

fn bxl(registers: &mut Vec<i64>, operand: i64) {
    registers[1] ^= operand;
}

fn bst(registers: &mut Vec<i64>, operand: i64) {
    registers[1] = combo_operand(registers, operand) % 8;
}

fn jnz(registers: &mut Vec<i64>, operand: i64) -> Option<i64> {
    if registers[0] != 0 {
        Some(operand)
    } else {
        None
    }
}

fn bxc(registers: &mut Vec<i64>, _operand: i64) {
    registers[1] ^= registers[2];
}

fn out(registers: &mut Vec<i64>, operand: i64) -> i64 {
    combo_operand(registers, operand) % 8
}

fn bdv(registers: &mut Vec<i64>, operand: i64) {
    registers[1] = registers[0] / 2_i64.pow(combo_operand(registers, operand) as u32);
}
fn cdv(registers: &mut Vec<i64>, operand: i64) {
    registers[2] = registers[0] / 2_i64.pow(combo_operand(registers, operand) as u32);
}

fn combo_operand(registers: &Vec<i64>, operand: i64) -> i64 {
    match operand {
        0..=3 => operand,
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        _ => panic!("invalid combo operand"),
    }
}

fn simulate(mut registers: Vec<i64>, program: &Vec<i64>) -> String {
    let mut inst_ptr = 0;
    let mut output = Vec::new();

    while (inst_ptr + 1) < program.len() {
        match program[inst_ptr] {
            0 => adv(&mut registers, program[inst_ptr + 1]),
            1 => bxl(&mut registers, program[inst_ptr + 1]),
            2 => bst(&mut registers, program[inst_ptr + 1]),
            3 => {
                if let Some(jmp) = jnz(&mut registers, program[inst_ptr + 1]) {
                    inst_ptr = jmp as usize;
                    continue;
                }
            }
            4 => bxc(&mut registers, program[inst_ptr + 1]),
            5 => output.push(out(&mut registers, program[inst_ptr + 1])),
            6 => bdv(&mut registers, program[inst_ptr + 1]),
            7 => cdv(&mut registers, program[inst_ptr + 1]),
            _ => panic!("invalid instruction"),
        }

        inst_ptr += 2;
    }

    output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn simulate_part2(mut registers: Vec<i64>, program: &Vec<i64>) -> bool {
    let mut inst_ptr = 0;
    let mut output = Vec::new();

    while (inst_ptr + 1) < program.len() {
        match program[inst_ptr] {
            0 => adv(&mut registers, program[inst_ptr + 1]),
            1 => bxl(&mut registers, program[inst_ptr + 1]),
            2 => bst(&mut registers, program[inst_ptr + 1]),
            3 => {
                if let Some(jmp) = jnz(&mut registers, program[inst_ptr + 1]) {
                    inst_ptr = jmp as usize;
                    continue;
                }
            }
            4 => bxc(&mut registers, program[inst_ptr + 1]),
            5 => {
                // only kill it if the lengths don't match and another output
                // is attempted because a program may be a perfect match
                // and continue to do some irrelevant steps
                // also protects the later index access
                if output.len() >= program.len() {
                    return false;
                }
                let to_push = out(&mut registers, program[inst_ptr + 1]);

                // early stopping speed up
                // no way to remove output so kill as soon as it diverges
                if to_push != program[output.len()] {
                    return false;
                }
                output.push(to_push);
            }
            6 => bdv(&mut registers, program[inst_ptr + 1]),
            7 => cdv(&mut registers, program[inst_ptr + 1]),
            _ => panic!("invalid instruction"),
        }

        inst_ptr += 2;
    }

    output == *program
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let split: Vec<_> = input.split("\n\n").collect();

    let registers: Vec<i64> = split[0]
        .lines()
        .map(|line| line.split(": ").last().unwrap().parse().unwrap())
        .collect();
    // println!("{:?}", registers);

    let program: Vec<i64> = split[1]
        .lines()
        .flat_map(|line| {
            line.split(": ")
                .last()
                .unwrap()
                .split(",")
                .map(|v| v.parse().unwrap())
        })
        .collect();

    // println!("{:?}", program);

    println!("{}", simulate(registers.clone(), &program));

    // floor((x / (8 ^ len)) = 1
    // s.t. floor((x - 1) / (8 ^ len)) = 0
    // i.e. for Reg_A = 8 ^ len - 1, floor(Reg_A / (8 ^ len)) = 0
    // loop terminates after len iterations
    // for Reg_A > 8 ^ len - 1, terminate after > len iterations
    let lower = 8_i64.pow((program.len() - 1) as u32);
    let upper = 8_i64.pow(program.len() as u32) - 1;

    println!("{:?}", lower);
    println!("{:?}", upper);

    for i in lower..=upper {
        if (i - lower) % 1_000_000 == 0 {
            println!("{i}");
        }
        let mut reg = registers.clone();
        reg[0] = i;
        if simulate_part2(reg, &program) {
            println!("part 2: {}", i);
            break;
        }
    }
}
