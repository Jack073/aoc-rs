use itertools::Itertools;
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

fn part_two() -> usize {
    BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .map(|l| l.unwrap())
        .chunks(3)
        .into_iter()
        .map(|chunk| {
            let c = chunk
                .map(|line| {
                    line.split_ascii_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            vec![
                [c[0][0], c[1][0], c[2][0]],
                [c[0][1], c[1][1], c[2][1]],
                [c[0][2], c[1][2], c[2][2]],
            ]
        })
        .flatten()
        .filter_map(|line| {
            if [(0, 1, 2), (0, 2, 1), (1, 2, 0)]
                .iter()
                .all(|sides| line[sides.0] + line[sides.1] > line[sides.2])
            {
                Some(true)
            } else {
                None
            }
        })
        .count()
}

fn main() {
    println!("{}", part_two());
}
