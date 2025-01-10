use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, usize,
};

#[derive(Debug, Default)]
struct Node {
    children: HashMap<char, Node>,
    is_terminal: bool,
}

#[derive(Debug)]
struct Trie {
    root: Node,
}

impl Trie {
    fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    fn insert(&mut self, key: &str) {
        let mut cur = &mut self.root;
        for c in key.chars() {
            cur = cur.children.entry(c).or_insert(Node::default());
        }

        cur.is_terminal = true;
    }

    fn find_terminal_indices(&self, key: &str) -> Vec<usize> {
        let mut ret = Vec::new();

        let mut cur = &self.root;
        for (i, c) in key.chars().enumerate() {
            if !cur.children.contains_key(&c) {
                break;
            }
            cur = &cur.children[&c];

            if cur.is_terminal {
                ret.push(i);
            }
        }

        ret
    }

    fn find(&self, key: &str) -> Option<&Node> {
        let mut cur = &self.root;
        for c in key.chars() {
            if !cur.children.contains_key(&c) {
                return None;
            }
            cur = &cur.children[&c];
        }

        Some(cur)
    }

    fn contains(&self, key: &str) -> bool {
        let mut cur = &self.root;
        for c in key.chars() {
            if !cur.children.contains_key(&c) {
                return false;
            }
            cur = &cur.children[&c];
        }
        true
    }

    fn contains_terminal(&self, key: &str) -> bool {
        let mut cur = &self.root;
        for c in key.chars() {
            if !cur.children.contains_key(&c) {
                return false;
            }
            cur = &cur.children[&c];
        }

        cur.is_terminal
    }
}

impl<'a> FromIterator<&'a str> for Trie {
    fn from_iter<T: IntoIterator<Item = &'a str>>(iter: T) -> Self {
        let mut trie = Trie::new();

        for i in iter {
            trie.insert(i);
        }

        trie
    }
}

fn count_word_spellings(alphabet: &Trie, word: &str) -> i64 {
    let mut counts = HashMap::new();
    _count_word_spellings(alphabet, word, &mut counts)
}

fn _count_word_spellings<'a>(
    alphabet: &Trie,
    word: &'a str,
    counts: &mut HashMap<&'a str, i64>,
) -> i64 {
    let mut total = 0;

    // this is a necessary speed up
    // otherwise a lot of recursions end up recomputing the tailends
    // that we already know are solvable/unsolvable
    // note that tailends are isolated, so to speak, so the path taken to reach known tailend has
    // no effect on its result
    if counts.contains_key(word) {
        return counts[word];
    }

    // terminal indices match the corresponding char in the word
    // a trie is an improvement over a hashmap containing only the tokens here
    // because we can quickly kill a search if, say, the first letter of word
    // will not match any token because we have tokenized each letter in the trie
    // Then, the actual tokens are marked as "terminal"
    // if we match a terminal token, then we try to solve the remainder of the word
    // note that the way the boundaries work, a word may only be matched by a token which entirely
    // spans it, or it might only be matched with single letter tokens; there isn't a good intution
    // for "it didn't match the substring so we can knock out the encompassing case"or vice versa
    let term_idx = alphabet.find_terminal_indices(word);
    for i in term_idx {
        // add 1 because we will slice everything from the right of the alphabet token
        if i + 1 < word.len() {
            total += _count_word_spellings(alphabet, &word[(i + 1)..], counts);
        } else {
            total += 1;
        }
    }

    if !counts.contains_key(word) {
        counts.insert(word, total);
    }

    total
}

// fn is_word_spellable(alphabet: &Trie, word: &str) -> bool {
//     assert!(word.len() > 0);
//
//     for end in 1..(word.len() + 1) {
//         // println!("WORD: {}", word);
//         // println!("SLICE: {}", &word[0..end]);
//         if let Some(node) = alphabet.find(&word[0..end]) {
//             if node.is_terminal {
//                 if end == word.len()
//                     || (end < word.len() && is_word_spellable(alphabet, &word[end..]))
//                 {
//                     return true;
//                 }
//             }
//         } else {
//             break;
//         }
//     }
//
//     false
// }

const INPUT_PATH: &str = "input/input.txt";
fn main() {
    let input = fs::read_to_string(INPUT_PATH).unwrap();
    let split = input.split("\n\n").collect::<Vec<_>>();

    let alphabet = split[0]
        .lines()
        .flat_map(|line| line.split(", "))
        .collect::<Trie>();

    let words = split[1].lines().collect::<Vec<_>>();

    // println!("{:?}", alphabet);
    // println!("{:?}", words);

    let n_possible = words
        .iter()
        .filter(|word| count_word_spellings(&alphabet, word) > 0)
        .count();

    println!("part 1: {}", n_possible);

    let n_ways = words
        .iter()
        .map(|word| {
            // println!("{word} {}", count_word_spellings(&alphabet, word));
            count_word_spellings(&alphabet, word)
        })
        .sum::<i64>();

    println!("part 2: {}", n_ways);
}
