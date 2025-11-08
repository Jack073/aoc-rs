use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|l| match l {
            Ok(l) => Some(l.parse::<usize>().unwrap_or(0)),
            _ => None,
        })
        .map(|n| (n / 3) - 2)
        .sum()
}

fn main() {
    println!("{} ", part_one());
}
