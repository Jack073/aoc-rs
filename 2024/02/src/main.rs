use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<isize>>(),
            ),
            Err(_) => None,
        })
        .filter(|row| {
            let deltas = row.windows(2).map(|w| w[0] - w[1]).collect::<Vec<isize>>();
            deltas.windows(2).all(|w| w[0].signum() == w[1].signum())
                && deltas.iter().all(|d| match d {
                    -3 | -2 | -1 | 1 | 2 | 3 => true,
                    _ => false,
                })
        })
        .count()
}

fn main() {
    println!("{}", part_one());
}
