use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    let nums = BufReader::new(File::open("input.txt").unwrap())
        .split(b',')
        .map(Result::unwrap)
        .map(String::from_utf8)
        .map(Result::unwrap)
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();

    (min..=max)
        .map(|n| nums.iter().map(|&x| x.abs_diff(n)).sum::<usize>())
        .min()
        .unwrap()
}

fn main() {
    println!("{}", part_one());
}
