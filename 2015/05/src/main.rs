use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("input file"))
        .lines()
        .filter_map(|l| match l {
            Ok(s) => Some(s),
            Err(_) => None,
        })
        .filter(|s| !["ab", "cd", "pq", "xy"].iter().any(|c| s.contains(c)))
        .filter(|s| {
            s.chars()
                .filter(|c| ['a', 'e', 'i', 'o', 'u'].iter().any(|n| c == n))
                .count()
                >= 3
        })
        .filter(|s| {
            s.chars()
                .chunk_by(|&k| k)
                .into_iter()
                .any(|i| i.1.count() >= 2)
        })
        .count()
}

fn main() {
    println!("{}", part_one());
}
