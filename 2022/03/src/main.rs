use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let split = line.split_at(line.len() / 2);

            [split.0, split.1]
                .map(|compartment| {
                    compartment.chars().fold(0u64, |acc, c| {
                        if c.is_uppercase() {
                            acc | (1 << (c as u64 - 'A' as u64) + 27)
                        } else {
                            acc | (1 << (c as u64 - 'a' as u64) + 1)
                        }
                    })
                })
                .into_iter()
                .reduce(|acc, new| acc & new)
                .unwrap()
                .trailing_zeros() as usize
        })
        .sum()
}


fn main() {
    println!("{}", part_one());
}
