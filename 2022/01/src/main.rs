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

fn part_two() -> usize {
    let list = BufReader::new(File::open("input.txt").expect("input.txt error"))
        .lines()
        .map(|l| l.unwrap())
        .chunk_by(|line| line.is_empty())
        .into_iter()
        .filter(|g| !g.0)
        .map(|line| line.1.map(|c| c.parse::<usize>().unwrap()).sum::<usize>())
        .sorted()
        .rev()
        .collect_vec();

    list[0] + list[1] + list[2]
}

fn main() {
    println!("{}", part_two());
}
