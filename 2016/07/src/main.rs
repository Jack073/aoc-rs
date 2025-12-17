use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn abba_check(window: (char, char, char, char)) -> bool {
    window.0 != window.1 && window.0 == window.3 && window.1 == window.2
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|x| {
            let mut has_abba = false;

            for (i, split) in x.rsplit(|c| c == '[' || c == ']').enumerate() {
                if split.chars().tuple_windows().any(abba_check) {
                    if i % 2 == 0 {
                        has_abba = true;
                    } else {
                        return false;
                    }
                }
            }

            has_abba
        })
        .count()
}

fn main() {
    println!("{}", part_one());
}
