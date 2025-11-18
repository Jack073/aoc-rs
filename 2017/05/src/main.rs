use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    let mut instructions = BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .filter_map(|l| match l {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .map(|l| l.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    let mut jumps = 0;
    let mut pos = 0;

    while pos < instructions.len() {
        jumps+=1;

        let next_jump = &mut instructions[pos];

        pos = pos.saturating_add_signed(*next_jump);
        *next_jump += 1;
    }

    jumps
}

fn main() {
    println!("{}", part_one());
}
