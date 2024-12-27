use std::fs;

fn count_vert(grid: &Vec<Vec<char>>, s: &str) -> i32 {
    let mut sum = 0;
    for i in 0..(grid.len() - s.len() + 1) {
        for j in 0..(grid[0].len()) {
            let grid_s: String = (0..s.len()).map(|k| grid[i + k][j]).collect();
            if s == grid_s {
                sum += 1;
            }
        }
    }
    println!("{sum}");

    return sum;
}

fn count_horiz(grid: &Vec<Vec<char>>, s: &str) -> i32 {
    let mut sum = 0;
    for i in 0..(grid.len()) {
        for j in 0..(grid[0].len() - s.len() + 1) {
            let grid_s: String = (0..s.len()).map(|k| grid[i][j + k]).collect();
            if s == grid_s {
                sum += 1;
            }
        }
    }
    println!("{sum}");

    return sum;
}

fn count_tl_br_diag(grid: &Vec<Vec<char>>, s: &str) -> i32 {
    let mut sum = 0;
    for i in 0..(grid.len() - s.len() + 1) {
        for j in 0..(grid[0].len() - s.len() + 1) {
            let grid_s: String = (0..s.len()).map(|k| grid[i + k][j + k]).collect();
            if s == grid_s {
                sum += 1;
            }
        }
    }
    println!("{sum}");

    return sum;
}

fn count_bl_tr_diag(grid: &Vec<Vec<char>>, s: &str) -> i32 {
    let mut sum = 0;
    for i in (s.len() - 1)..grid.len() {
        for j in 0..(grid[0].len() - s.len() + 1) {
            let grid_s: String = (0..s.len()).map(|k| grid[i - k][j + k]).collect();
            if s == grid_s {
                sum += 1;
            }
        }
    }
    println!("{sum}");

    return sum;
}

fn count_occurrences(grid: &str, s: &str) -> i32 {
    let grid: Vec<Vec<_>> = grid.lines().map(|line| line.chars().collect()).collect();

    let s_rev: String = s.chars().rev().collect();

    return count_vert(&grid, s)
        + count_horiz(&grid, s)
        + count_tl_br_diag(&grid, s)
        + count_bl_tr_diag(&grid, s)
        + count_vert(&grid, &s_rev)
        + count_horiz(&grid, &s_rev)
        + count_tl_br_diag(&grid, &s_rev)
        + count_bl_tr_diag(&grid, &s_rev);
}

fn count_occurrences_part_2(grid: &str) -> i32 {
    let grid: Vec<Vec<_>> = grid.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            // println!(
            //     "{} {}\n {} \n{} {}",
            //     grid[i - 1][j - 1],
            //     grid[i - 1][j + 1],
            //     grid[i][j],
            //     grid[i + 1][j - 1],
            //     grid[i + 1][j + 1]
            // );
            if grid[i][j] == 'A'
                && ((grid[i - 1][j - 1] == 'S' && grid[i + 1][j + 1] == 'M')
                    || (grid[i - 1][j - 1] == 'M' && grid[i + 1][j + 1] == 'S'))
                && ((grid[i - 1][j + 1] == 'S' && grid[i + 1][j - 1] == 'M')
                    || (grid[i - 1][j + 1] == 'M' && grid[i + 1][j - 1] == 'S'))
            {
                sum += 1;
            }
        }
    }
    println!("{sum}");

    sum
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    println!("part 1: {}", count_occurrences(&input, "XMAS"));

    println!("part 2: {}", count_occurrences_part_2(&input));
}
