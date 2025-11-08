use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_frequencies() -> impl Iterator<Item = [u8; 26]> {
    BufReader::new(File::open("input.txt").expect("input file"))
        .lines()
        .filter_map(|l| if let Ok(n) = l { Some(n) } else { None })
        .map(|l| {
            let mut frequencies = [0u8; 26];
            l.chars().for_each(|c| {
                frequencies[(c as u8 - b'a') as usize] += 1;
            });
            frequencies
        })
}

fn part_one() -> usize {
    let counts = get_frequencies()
        .map(|item| {
            (if item.iter().any(|&n| n == 2) {
                1
            } else {
                 0
            }, if item.iter().any(|&n| n == 3) {
                1
            } else {
                0
            })
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    counts.0 * counts.1
}

fn main() {
    println!("{}", part_one());
}
