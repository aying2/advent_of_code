use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn is_subset<T>(sup: &[T], sub: &[T]) -> bool
where
    T: PartialEq,
{
    for elem in sub {
        if !sup.contains(elem) {
            return false;
        }
    }

    true
}

fn intersect<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: PartialEq + Copy,
{
    let mut v = Vec::new();
    for elem in a {
        if b.contains(elem) {
            v.push(*elem)
        }
    }

    v
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let mut prereqs = HashMap::new();

    let mut rules_flag = true;
    let updates = input
        .lines()
        .filter_map(|line| {
            if line.is_empty() {
                rules_flag = false;
                return None;
            }
            if rules_flag {
                let (prereq, page) = line.split_once("|").unwrap();
                prereqs
                    .entry(page.parse::<i32>().unwrap())
                    .or_insert(Vec::new())
                    .push(prereq.parse::<i32>().unwrap());
                return None;
            } else {
                return Some(
                    line.split(",")
                        .map(|e| e.parse::<i32>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
        })
        .collect::<Vec<_>>();

    // println!("{:?}", prereqs);
    // println!("{:?}", updates);
    let mut mid_sum = 0;
    for update in updates.iter() {
        let mut add_flag = true;
        for (i, page) in update.iter().enumerate() {
            // println!("{:?}", page);

            // skip if no prereq
            let prereq = match prereqs.get(page) {
                Some(v) => v,
                None => {
                    continue;
                }
            };
            let preceding = &update[..i];
            // println!("{:?}", preceding);
            // println!("{:?}", prereq);

            // intersect because only wrong if needed prereq is somewhere else in the update
            if !is_subset(preceding, &intersect(prereq, update)) {
                add_flag = false;
                break;
            }
        }

        if add_flag {
            // println!("MID {}", update[update.len() / 2]);
            mid_sum += update[update.len() / 2];
        }
    }

    println!("part 1: {mid_sum}");

    // println!("{:?}", prereqs);
    // println!("{:?}", updates);
    let mut mid_sum = 0;
    for mut update in updates {
        let mut add_flag = true;
        for (i, page) in update.iter().enumerate() {
            // println!("{:?}", page);
            let prereq = match prereqs.get(page) {
                Some(v) => v,
                None => {
                    continue;
                }
            };
            let preceding = &update[..i];
            // println!("{:?}", preceding);
            // println!("{:?}", prereq);
            if !is_subset(preceding, &intersect(prereq, &update)) {
                add_flag = false;
                break;
            }
        }

        // it is an abomination
        update.sort_unstable_by(|a, b| {
            let a_prereqs = prereqs.get(a);
            let b_prereqs = prereqs.get(b);

            if a_prereqs.is_none() && b_prereqs.is_none() {
                return Ordering::Equal;
            } else if a_prereqs.is_none() {
                return Ordering::Less;
            } else if b_prereqs.is_none() {
                return Ordering::Greater;
            }

            if a_prereqs.unwrap().contains(b) && b_prereqs.unwrap().contains(a) {
                return Ordering::Equal;
            } else if b_prereqs.unwrap().contains(a) {
                return Ordering::Less;
            } else if a_prereqs.unwrap().contains(b) {
                return Ordering::Greater;
            }

            Ordering::Equal
        });

        if !add_flag {
            // println!("MID {}", update[update.len() / 2]);
            mid_sum += update[update.len() / 2];
        }
    }

    println!("part 2: {mid_sum}");
}
