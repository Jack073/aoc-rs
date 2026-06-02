use std::fs::File;
use std::io::{BufRead, BufReader};

fn contains(s1: usize, e1: usize, s2: usize, e2: usize) -> bool {
    s1 <= s2 && e1 >= e2 && s2 <= e1
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("input.txt error"))
        .lines()
        .map(|l| {
            l.unwrap()
                .split(|c| c == '-' || c == ',')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|l| contains(l[0], l[1], l[2], l[3]) || contains(l[2], l[3], l[0], l[1]))
        .count()
}

fn main() {
    println!("{}", part_one());
}
