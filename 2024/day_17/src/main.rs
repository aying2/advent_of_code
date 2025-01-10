use std::{fs, usize};

fn adv(registers: &mut Vec<i64>, operand: i64) {
    registers[0] >>= combo_operand(registers, operand);
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
    registers[1] = registers[0] >> combo_operand(registers, operand);
}
fn cdv(registers: &mut Vec<i64>, operand: i64) {
    registers[2] = registers[0] >> combo_operand(registers, operand);
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

fn simulate(mut registers: Vec<i64>, program: &Vec<i64>) -> Vec<i64> {
    let mut inst_ptr = 0;
    let mut output = Vec::new();

    while (inst_ptr + 1) < program.len() {
        // println!("inst: {}", program[inst_ptr]);
        // println!("A: {:#032b}", registers[0]);
        // println!("B: {:#032b}", registers[1]);
        // println!("C: {:#032b}", registers[2]);
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
}

// the solution relies on the fact that Reg A's values only depend on its starting value and the
// literal, so the starting value determines how many output digits are produced
// also the "jump" pattern here is relatively simple, essentially looping until Reg A is 0
const ADV_LITERAL: usize = 3;

fn find_earliest_copy_reg_a(registers: &Vec<i64>, program: &Vec<i64>) -> Option<i64> {
    _find_earliest_copy_reg_a(registers, program, 0, 0)
}

fn _find_earliest_copy_reg_a(
    registers: &Vec<i64>,
    program: &Vec<i64>,
    offset: i64,
    depth: usize,
) -> Option<i64> {
    // use a depth-based recursion because a stack based solution would have to keeptrack of the
    // offsets and the desired digits at each level
    if depth > (program.len() - 1) {
        return Some(offset);
    }

    // NOTE: all levels except the first start from 0 because shifting 0 means we would repeat
    // levels; however, this also means that the case of a program of "0" for which a 0 in Reg A
    // outputs 0 won't be handled correctly
    let start = if depth == 0 { 1 } else { 0 };

    // the key insight is that the each digit of the program can be solved for separately starting
    // from the back of the program because of the shifting of Reg A from the ADV operation.
    // For example, with a shift of 3, we know that Reg A = 1 results in a single output before
    // the loop terminates, and if that output is 5, then any initial Reg A with 001 in the leftmost
    // block of 3 bits, will have a final output of 5. Then, to get the second to last digit of the
    // program correct, we need to test 001XXX and see which initial Reg A have this digit in the
    // first output element (we let simulate run all the way through but only care about the first
    // element since we know what the later elements must be from the previous level; possible
    // speed up to early stop). And so on for the other levels. But the tricky bit is that
    // multiple of the 8 cases which are tested at each level may correspond to the correct digit
    // and some of them may fail only in the later levels (the way in which the new bits of Reg A
    // affect Reg B and C and therefore the output cannot be easily predicted). Thus, we do a depth
    // first search from the first matching case (which will lead to the earliest copy) and
    // if it fails later, we must backtrack to these other cases
    for j in start..(1 << ADV_LITERAL) {
        let mut reg = registers.clone();
        let reg_a = (offset << ADV_LITERAL) + j;
        reg[0] = reg_a;
        let output = simulate(reg, program);

        if output[0] == program[program.len() - 1 - depth] {
            let opt = _find_earliest_copy_reg_a(registers, program, reg_a, depth + 1);
            if opt.is_some() {
                return opt;
            }
        }
    }

    None
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

    println!(
        "part 1: {}",
        simulate(registers.clone(), &program)
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    println!(
        "part 2: {:?}",
        find_earliest_copy_reg_a(&registers, &program)
    );

    // // floor((x / (8 ^ len)) = 1
    // // s.t. floor((x - 1) / (8 ^ len)) = 0
    // // i.e. for Reg_A = 8 ^ len - 1, floor(Reg_A / (8 ^ len)) = 0
    // // loop terminates after len iterations
    // // for Reg_A > 8 ^ len - 1, terminate after > len iterations
    // let lower = 8_i64.pow((program.len() - 1) as u32);
    // let upper = 8_i64.pow(program.len() as u32) - 1;
    //
    // println!("{:?}", lower);
    // println!("{:?}", upper);
    //
    // // inspect patterns between Reg A and the output for small programs
    // for i in 1..=511 {
    //     let mut reg = registers.clone();
    //     reg[0] = i;
    //
    //     println!("{i} : {:?}", simulate(reg, &program));
    // }
}
