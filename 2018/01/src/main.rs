use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_frequencies() -> impl Iterator<Item = isize> {
    BufReader::new(File::open("input.txt").expect("missing input file"))
        .lines()
        .filter_map(|n| if let Ok(s) = n { Some(s) } else { None })
        .map(|n| n.trim_start_matches("+").parse::<isize>().unwrap_or(0))
}

fn part_one() -> isize {
    get_frequencies().sum()
}

fn part_two() -> isize {
    let mut set = HashSet::new();

    get_frequencies()
        .collect::<Vec<isize>>()
        .iter()
        .cycle()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        }).find(|s| !set.insert(*s))
        .unwrap()
}

fn main() {
    println!("{}", part_two());
}
