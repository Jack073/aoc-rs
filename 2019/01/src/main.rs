use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|l| match l {
            Ok(l) => Some(l.parse::<usize>().unwrap_or(0)),
            _ => None,
        })
        .map(|n| (n / 3) - 2)
        .sum()
}

fn recursive_fuel_calculation(fuel: usize) -> usize {
    if fuel < 6 {
        0
    } else {
        let cost = (fuel / 3) - 2;
        cost + recursive_fuel_calculation(cost)
    }
}

fn part_two() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .filter_map(|l| match l {
            Ok(l) => Some(l.parse::<usize>().unwrap_or(0)),
            _ => None,
        })
        .map(recursive_fuel_calculation)
        .sum()
}
fn main() {
    println!("{} ", part_two());
}
