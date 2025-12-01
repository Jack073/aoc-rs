use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    let mut point = 50;
    let mut hit_zero = 0;

    for d in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let sign = l.chars().nth(0).unwrap();
            let num = l
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<isize>()
                .unwrap();

            if sign == 'L' { -num } else { num }
        })
    {
        point = (point + d) % 100;
        if point == 0 {
            hit_zero += 1;
        }
    }

    hit_zero
}

fn part_two() -> usize {
    let mut point = 50;
    let mut hit_zero = 0;

    for d in BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let sign = l.chars().nth(0).unwrap();
            let num = l
                .chars()
                .skip(1)
                .collect::<String>()
                .parse::<isize>()
                .unwrap();

            if sign == 'L' { -num } else { num }
        })
    {
        let delta = d.signum();
        for _ in (0..d.abs()) {
            point = (point + delta) % 100;
            if point == 0 {
                hit_zero += 1;
            }
        }
    }

    hit_zero
}

fn main() {
    println!("{}", part_two());
}
