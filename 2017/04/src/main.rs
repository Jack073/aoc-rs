use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .filter_map(|l| match l {
            Ok(l) => Some(l),
            Err(_) => None,
        })
        .filter(|l| {
            l.split_ascii_whitespace()
                .collect::<Vec<&str>>()
                .into_iter()
                .sorted()
                .chunk_by(|&k| k)
                .into_iter()
                .all(|g| g.1.count() == 1)
        })
        .count()
}

fn main() {
    println!("{}", part_one());
}
