// Write a program to count the frequencies of unique words from standard input,
// then print them out with their frequencies, ordered most frequent first.
// For example, given this input:
//
// The foo the foo the
// defenestration the
//
// The program should print the following:
//
// the 4
// foo 2
// defenestration 1

use std::{collections::HashMap, io::Read};

fn main() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_to_string(&mut buffer)
        .expect("failed to read stdin");
    let words = buffer
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .collect::<Vec<_>>();

    let mut map: HashMap<String, u32> = HashMap::new();
    for w in words {
        let ent = map.entry(w).or_insert(0);
        *ent += 1;
    }

    let mut v = map.into_iter().collect::<Vec<_>>();
    v.sort_by(|a, b| b.1.cmp(&a.1));

    for (w, c) in v.iter() {
        println!("{w} {c}");
    }
}
