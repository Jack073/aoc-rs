use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_lines() -> impl Iterator<Item = String> {
    BufReader::new(File::open("input.txt").expect("input file"))
        .lines()
        .filter_map(|l| if let Ok(n) = l { Some(n) } else { None })
}

fn get_frequencies() -> impl Iterator<Item = [u8; 26]> {
    get_lines().map(|l| {
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
            (
                if item.iter().any(|&n| n == 2) { 1 } else { 0 },
                if item.iter().any(|&n| n == 3) { 1 } else { 0 },
            )
        })
        .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

    counts.0 * counts.1
}

fn part_two() -> String {
    let lines = get_lines().collect::<Vec<_>>();

    for (i, line) in lines.iter().enumerate() {
        for j in (i + 1)..lines.len() {
            let line2 = &lines[j];

            let mutual = line.chars().enumerate().filter_map(|(index, c)| {
                if line2.chars().nth(index) == Some(c) {
                    Some(c)
                } else {
                    None
                }
            });

            if mutual.clone().count() < line.len() - 1 {
                continue;
            }

            return mutual.collect::<String>();
        }
    }

    String::new()
}

fn main() {
    println!("{}", part_two());
}
