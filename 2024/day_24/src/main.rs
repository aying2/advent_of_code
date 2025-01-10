use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    hash::Hash,
    usize, vec,
};

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Gate<'a> {
    out: &'a str,
    in1: &'a str,
    in2: &'a str,
    op: Op,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Op {
    OR,
    AND,
    XOR,
}

impl Op {
    fn to_string(&self) -> &str {
        match self {
            Op::OR => "OR",
            Op::AND => "AND",
            Op::XOR => "XOR",
        }
    }
}

fn execute_op(in1: i32, in2: i32, op: Op) -> i32 {
    match op {
        Op::OR => in1 | in2,
        Op::AND => in1 & in2,
        Op::XOR => in1 ^ in2,
    }
}

fn solve<'a>(
    mut values: HashMap<&'a str, i32>,
    children: &'a HashMap<&str, Gate>,
) -> HashMap<&'a str, i32> {
    let parents = children.keys().collect::<HashSet<_>>();

    while let Some(root) = parents
        .difference(&values.keys().collect::<HashSet<_>>())
        .next()
    {
        solve_subtree(&root, &mut values, children);
    }

    values
}

fn solve_subtree<'a>(
    parent: &'a str,
    values: &mut HashMap<&'a str, i32>,
    children: &'a HashMap<&str, Gate>,
) {
    if !values.contains_key(children[parent].in1) {
        solve_subtree(children[parent].in1, values, children);
    }

    if !values.contains_key(children[parent].in2) {
        solve_subtree(children[parent].in2, values, children);
    }

    if values.contains_key(children[parent].in1) && values.contains_key(children[parent].in2) {
        values.insert(
            parent,
            execute_op(
                values[children[parent].in1],
                values[children[parent].in2],
                children[parent].op,
            ),
        );
    } else {
        panic!("unsolvable subtree");
    }
}

fn convert_z_bits_to_decimal(values: HashMap<&str, i32>) -> i64 {
    let mut z_bits = values
        .into_iter()
        .filter(|(k, _)| k.starts_with("z"))
        .collect::<Vec<_>>();

    z_bits.sort_unstable_by_key(|&(k, _)| k);

    i64::from_str_radix(
        &z_bits
            .into_iter()
            // reverse for little endian
            .rev()
            .map(|(_, v)| v.to_string())
            .collect::<Vec<_>>()
            .concat(),
        2,
    )
    .unwrap()
}

fn get_descendants<'a>(children: &HashMap<&str, Gate<'a>>, z_out: &str) -> HashSet<Gate<'a>> {
    let mut descendants = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(z_out);

    while let Some(child) = queue.pop_front() {
        if children.contains_key(child) {
            descendants.insert(children[&child]);
            queue.push_back(children[&child].in1);
            queue.push_back(children[&child].in2);
        }
    }

    descendants
}

fn is_valid_half_adder(z_out: &str, children: &HashMap<&str, Gate>) -> bool {
    let x_in = z_out.replace("z", "x");
    let y_in = z_out.replace("z", "y");

    let n = z_out[1..].parse::<usize>().unwrap() + 1;

    let input_names = (0..n)
        .flat_map(|i| [format!("x{:02}", i), format!("y{:02}", i)].into_iter())
        .collect::<Vec<_>>();

    for x in [0, 1] {
        for y in [0, 1] {
            let mut values = input_names
                .iter()
                .map(|name| (name.as_str(), 0))
                .collect::<HashMap<_, _>>();
            *values.get_mut(x_in.as_str()).unwrap() = x;
            *values.get_mut(y_in.as_str()).unwrap() = y;

            solve_subtree(z_out, &mut values, children);

            if values[z_out] != x ^ y {
                return false;
            }
        }
    }

    true
}

fn is_valid_full_adder(z_out: &str, children: &HashMap<&str, Gate>) -> bool {
    let idx = z_out[1..].parse::<usize>().unwrap();

    let x_in = z_out.replace("z", "x");
    let y_in = z_out.replace("z", "y");

    let x_prev_in = format!("x{:02}", idx - 1);
    let y_prev_in = format!("y{:02}", idx - 1);

    let n = idx + 1;

    let input_names = (0..n)
        .flat_map(|i| [format!("x{:02}", i), format!("y{:02}", i)].into_iter())
        .collect::<Vec<_>>();

    for x in [0, 1] {
        for y in [0, 1] {
            for x_prev in [0, 1] {
                for y_prev in [0, 1] {
                    let mut values = input_names
                        .iter()
                        .map(|name| (name.as_str(), 0))
                        .collect::<HashMap<_, _>>();
                    *values.get_mut(x_in.as_str()).unwrap() = x;
                    *values.get_mut(y_in.as_str()).unwrap() = y;

                    *values.get_mut(x_prev_in.as_str()).unwrap() = x_prev;
                    *values.get_mut(y_prev_in.as_str()).unwrap() = y_prev;

                    let c_in = x_prev & y_prev;

                    solve_subtree(z_out, &mut values, children);

                    if values[z_out] != x ^ y {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn get_descendant_diffs<'a>(children: &HashMap<&str, Gate<'a>>) -> Vec<HashSet<Gate<'a>>> {
    let mut z_outs = children
        .keys()
        .filter(|k| k.starts_with("z"))
        .collect::<Vec<_>>();

    z_outs.sort_unstable();

    let mut descendant_diffs = Vec::new();
    let mut descendants_cum = HashSet::new();

    for z_out in z_outs {
        let descendants = get_descendants(children, z_out);
        descendant_diffs.push(descendants.difference(&descendants_cum).copied().collect());
        descendants_cum = descendants_cum.union(&descendants).copied().collect();
    }

    descendant_diffs
}

// fn is_half_adder(children: &HashMap<&str, Gate>, z_out: &str) -> Option<&str> {}
//
// fn is_full_adder(children: &HashMap<&str, Gate>, z_out: &str) -> Option<&str> {
//     let x_in = z_out.replace("z", "x");
//     let y_in = z_out.replace("z", "y");
//
//     if children[z_out].op != Op::XOR {
//         return false;
//     }
//
//     if children[z_out].in1 == x_in || child == y_in {}
//
//     let mut xors = HashSet::new();
//
//     let mut stack = Vec::new();
//
//     stack.push(z_out);
//
//     while let Some(cur) = stack.pop() {
//         if children[cur].op == Op::XOR {
//             for child in [children[cur].in1, children[cur].in2] {
//                 if child == x_in || child == y_in {
//                     xors.insert(child);
//                 } else {
//                     stack.push(child);
//                 }
//             }
//         } else if children[cur].op == Op::OR {
//             xors.insert(cur);
//         }
//     }
//
//     None
// }

fn write_graphdot_whole(block: Vec<&str>, path: &str) {
    let mut graphdot = block[1]
        .lines()
        .flat_map(|line| {
            let split = line.split_whitespace().collect::<Vec<_>>();

            if let [in1, op, in2, _, out] = split[..] {
                let mut stmts = vec![
                    format!("{in1} -> {out}"),
                    format!("{in2} -> {out}"),
                    format!("{out} [label={op}, shape=box]"),
                ];
                if out.starts_with("z") {
                    stmts = [
                        stmts,
                        vec![
                            format!("{out} -> {out}_out"),
                            format!("{out}_out [label={out}, shape=diamond]"),
                        ],
                    ]
                    .concat();
                }
                stmts
            } else {
                panic!("bad gate definition");
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    graphdot = ["digraph G {", graphdot.as_str(), "}"].join("\n");
    fs::write(path, graphdot).unwrap();
}

fn write_graphdot_clusters(children: &HashMap<&str, Gate>, path: &str) {
    let descendant_diffs = get_descendant_diffs(children);

    let mut graphdot = descendant_diffs
        .into_iter()
        .enumerate()
        .flat_map(|(i, hashset)| {
            let mut diff = hashset
                .into_iter()
                .flat_map(|gate| {
                    let mut edge1 = format!("{} -> {}", gate.in1, gate.out);
                    if !gate.in1.starts_with("z") {
                        edge1.push_str(&format!(" [tooltip={}]", gate.in1));
                    }

                    let mut edge2 = format!("{} -> {}", gate.in2, gate.out);
                    if !gate.in2.starts_with("z") {
                        edge2.push_str(&format!(" [tooltip={}]", gate.in2));
                    }

                    let mut stmts = vec![
                        edge1,
                        edge2,
                        format!("{} [label={}, shape=box]", gate.out, gate.op.to_string()),
                    ];
                    if gate.out.starts_with("z") {
                        stmts = [
                            stmts,
                            vec![
                                format!("{} -> {}_out", gate.out, gate.out),
                                format!("{}_out [label={}, shape=diamond]", gate.out, gate.out),
                            ],
                        ]
                        .concat();
                    }
                    stmts
                })
                .collect::<Vec<_>>();

            diff.insert(0, format!("subgraph cluster{:02} {{", i));
            diff.push(format!("label=cluster{:02}", i));
            diff.push("}".to_string());

            diff
        })
        .collect::<Vec<_>>();

    graphdot.insert(0, "digraph G {".to_string());
    graphdot.push("}".to_string());
    fs::write(path, graphdot.join("\n")).unwrap();
}

fn swap_children(children: &mut HashMap<&str, Gate>, out1: &str, out2: &str) {
    (
        *children.get_mut(out1).unwrap(),
        *children.get_mut(out2).unwrap(),
    ) = (children[out2], children[out1]);

    (
        children.get_mut(out1).unwrap().out,
        children.get_mut(out2).unwrap().out,
    ) = (children[out2].out, children[out1].out);
}

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let block = input.split("\n\n").collect::<Vec<_>>();

    let values = block[0]
        .lines()
        .map(|line| {
            let split = line.split(": ").collect::<Vec<_>>();
            (split[0], split[1].parse::<i32>().unwrap())
        })
        .collect::<HashMap<_, _>>();

    // println!("{:?}", values);

    let mut children = HashMap::new();
    let mut parents = HashMap::new();

    for line in block[1].lines() {
        let split = line.split_whitespace().collect::<Vec<_>>();

        if let [in1, op, in2, _, out] = split[..] {
            let op = match op {
                "OR" => Op::OR,
                "AND" => Op::AND,
                "XOR" => Op::XOR,
                _ => panic!("bad op"),
            };
            children.insert(out, Gate { out, in1, in2, op });

            parents.entry(in1).or_insert(Vec::new()).push(out);
            parents.entry(in2).or_insert(Vec::new()).push(out);
        } else {
            panic!("bad gate definition");
        }
    }

    // println!("{:?}", children);

    let values = solve(values, &children);

    println!("part 1: {}", convert_z_bits_to_decimal(values));

    // honestly, one of the most annoying advent of codes problems because you can have many possible
    // structures both in terms of composition and permutation for an adder so you
    // might then instead opt to check behavior, i.e. it's an adder if the truth table matches but
    // because it remains tricky to identify the carry so you would have to check that just about
    // every input combination works to answer in the positive, building up bit by bit, and fixing
    // errors as they come
    // But again, because the adder structure is somewhat unpredictable, you
    // might need O(n^2) swaps (really, each output for the adder you've added against the
    // remaining unchecked).
    // I had some luck with comparing the descendants of an output bit against the cumulative
    // descendants of the previous bits for assigning "ownership" to nodes, and this roughly traces
    // out the divisions of the adders.
    // overall, I think this problem is a little unconstrained
    // practically, it seems that most swaps are pretty simple and between adjacent adders, don't
    // have multiple for the same adder, etc.
    // so I've opted for the manual visualization approach
    write_graphdot_clusters(&children, "graph_cluster.dot");

    let swap_pairs = [
        ("z12", "kwb"),
        ("z16", "qkf"),
        ("z24", "tgr"),
        ("jqn", "cph"),
    ];

    for swap_pair in swap_pairs {
        swap_children(&mut children, swap_pair.0, swap_pair.1);
    }

    write_graphdot_clusters(&children, "graph_cluster_swapped.dot");

    let mut swaps = swap_pairs
        .iter()
        .flat_map(|pair| [pair.0, pair.1].into_iter())
        .collect::<Vec<_>>();

    swaps.sort_unstable();

    println!("{}", swaps.join(","));
}
