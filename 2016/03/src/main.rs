use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .map(|l| l.unwrap())
        .filter_map(|line| {
            let l = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            if [(0, 1, 2), (0, 2, 1), (1, 2, 0)]
                .iter()
                .all(|sides| l[sides.0] + l[sides.1] > l[sides.2])
            {
                Some(true)
            } else {
                None
            }
        })
        .count()
}

fn main() {
    println!("{}", part_one());
}
