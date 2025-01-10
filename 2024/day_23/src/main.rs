use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn find_3_cycles<'a>(connections: &'a HashMap<&str, HashSet<&str>>) -> HashSet<[&'a str; 3]> {
    let mut cycles = HashSet::new();

    for node in connections.keys() {
        for neighbor in connections[node].iter() {
            for neighbor_neighbor in connections[neighbor].iter() {
                if connections[neighbor_neighbor].contains(node) {
                    let mut cycle = [*node, *neighbor, *neighbor_neighbor];
                    cycle.sort_unstable();

                    cycles.insert(cycle);
                }
            }
        }
    }

    cycles
}

fn count_3_cycles_with_t_start(connections: &HashMap<&str, HashSet<&str>>) -> i32 {
    find_3_cycles(connections)
        .into_iter()
        .map(|cycle| cycle.iter().any(|s| s.starts_with("t")) as i32)
        .sum()
}

fn find_max_clique<'a>(connections: &HashMap<&str, HashSet<&str>>) -> String {
    let mut maximal = Vec::new();

    bron_kerbosch(
        HashSet::new(),
        connections.keys().copied().collect(),
        HashSet::new(),
        connections,
        &mut maximal,
    );

    let mut max_clique = maximal
        .into_iter()
        .max_by(|x, y| x.len().cmp(&y.len()))
        .unwrap()
        .into_iter()
        .collect::<Vec<_>>();

    max_clique.sort_unstable();

    max_clique.join(",")
}

fn bron_kerbosch<'a>(
    r: HashSet<&'a str>,
    mut p: HashSet<&'a str>,
    mut x: HashSet<&'a str>,
    connections: &'a HashMap<&str, HashSet<&str>>,
    maximal: &mut Vec<HashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        maximal.push(r.clone());
    }

    for v in p.clone() {
        bron_kerbosch(
            r.union(&HashSet::from([v])).copied().collect(),
            p.intersection(&connections[v]).copied().collect(),
            x.intersection(&connections[v]).copied().collect(),
            connections,
            maximal,
        );

        p.remove(v);
        x.insert(v);
    }
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();

    let mut connections = HashMap::new();

    for line in input.lines() {
        let split = line.split("-").collect::<Vec<_>>();

        connections
            .entry(split[0])
            .or_insert(HashSet::new())
            .insert(split[1]);
        connections
            .entry(split[1])
            .or_insert(HashSet::new())
            .insert(split[0]);
    }

    // println!("{:?}", connections);
    // println!("{:?}", find_3_cycles(&connections));

    println!("part 1: {}", count_3_cycles_with_t_start(&connections));
    println!("part 2: {}", find_max_clique(&connections));

    find_max_clique(&connections);
}
