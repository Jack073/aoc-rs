use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("input.txt error"))
        .lines()
        .map(|l| l.unwrap())
        .chunk_by(|line| line.is_empty())
        .into_iter()
        .filter(|g| !g.0)
        .map(|line| line.1.map(|c| c.parse::<usize>().unwrap()).sum())
        .max()
        .unwrap()
}

fn main() {
    println!("{}", part_one());
}
