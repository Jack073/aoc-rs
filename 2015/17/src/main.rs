use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::BitAnd;

fn part_one() -> usize {
    // Quick and nasty - there's probably a better way to do this, but since there's only 20 inputs,
    // that works out at ~1 million combinations which is quick to brute force, especially when it
    // is cheap mathematical or binary operations.

    let sizes = BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .filter_map(|l| match l {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .map(|line| line.parse::<isize>().expect("invalid int"))
        .collect::<Vec<_>>();

    (0..(1 << 20))
        .map(|n| {
            (0..20)
                .filter_map(|and| {
                    if n.bitand(1 << and) > 0 {
                        Some(and)
                    } else {
                        None
                    }
                })
                .map(|index| sizes[index])
                .sum::<isize>()
        })
        .filter(|&c| c == 150)
        .count()
}

fn part_two() -> usize {
    let sizes = BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .filter_map(|l| match l {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .map(|line| line.parse::<isize>().expect("invalid int"))
        .collect::<Vec<_>>();

    *(0..(1 << 20))
        .map(|n| {
            let combination = (0..20)
                .filter_map(|and| {
                    if n.bitand(1 << and) > 0 {
                        Some(and)
                    } else {
                        None
                    }
                })
                .map(|index| sizes[index])
                .collect::<Vec<_>>();
            (combination.iter().sum::<isize>(), combination.len())
        })
        .filter(|&c| c.0 == 150)
        .map(|c| c.1)
        .fold(&mut [0usize; 20], |state, combination| {
            state[combination] += 1;
            state
        })
        .into_iter()
        .filter(|c| *c > &mut 0)
        .min()
        .unwrap()
}

fn main() {
    println!("{}", part_two());
}
