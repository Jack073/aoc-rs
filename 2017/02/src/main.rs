use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Sub;

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .filter_map(|l| match l {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .map(|line| {
            let (min, max) = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().expect("number parse fail"))
                .fold((usize::MAX, usize::MIN), |state, num| {
                    (state.0.min(num), state.1.max(num))
                });
            max.sub(min)
        })
        .sum()
}

fn main() {
    println!("{}", part_one());
}
