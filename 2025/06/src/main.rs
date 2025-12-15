use std::fs::File;
use std::io::{BufRead, BufReader};

enum Op {
    Add,
    Multiply,
}

impl Op {
    fn apply(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Op::Add => lhs + rhs,
            Op::Multiply => lhs * rhs,
        }
    }
}

fn part_one() -> usize {
    let mut lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let ops = lines[lines.len() - 1]
        .iter()
        .map(|n| match n.as_str() {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => panic!("invalid op"),
        })
        .collect::<Vec<_>>();

    let mut states = lines[0]
        .iter()
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (1..lines.len() - 1).for_each(|line_index| {
        lines[line_index].iter().enumerate().for_each(|(i, val)| {
            states[i] = ops[i].apply(states[i], val.parse().unwrap());
        })
    });

    states.iter().sum::<usize>()
}

fn main() {
    println!("{}", part_one());
}
