use std::fs::File;
use std::io::{BufRead, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref re: Regex = Regex::new("^[a-z ]+(\\d+),(\\d+) through (\\d+),(\\d+)$").unwrap();
}

enum Operation {
    On,
    Off,
    Toggle,
}

impl Operation {
    fn apply(&self, state: bool) -> bool {
        match self {
            Operation::On => true,
            Operation::Off => false,
            Operation::Toggle => !state,
        }
    }
}

struct Command {
    left_x: usize,
    left_y: usize,

    right_x: usize,
    right_y: usize,

    op: Operation,
}

impl Command {
    fn new(line: String) -> Self {
        let (_, [left_x, left_y, right_x, right_y]) =
            re.captures(&line).expect("no capture").extract();

        Self {
            left_x: left_x.parse().unwrap(),
            left_y: left_y.parse().unwrap(),
            right_x: right_x.parse().unwrap(),
            right_y: right_y.parse().unwrap(),
            op: match line.get(0..7) {
                Some("turn on") => Operation::On,
                Some("turn of") => Operation::Off,
                Some("toggle ") => Operation::Toggle,
                _ => panic!("invalid command"),
            },
        }
    }

    fn apply(&self, state: &mut Vec<Vec<bool>>) {
        for x in self.left_x..=self.right_x {
            for y in self.left_y..=self.right_y {
                state[x][y] = self.op.apply(state[x][y]);
            }
        }
    }
}

fn part_one() -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];

    let _ = BufReader::new(File::open("input.txt").expect("input file"))
        .lines()
        .filter_map(|l| match l {
            Ok(s) => Some(s),
            Err(_) => None,
        })
        .map(|line| Command::new(line))
        .for_each(|c| c.apply(&mut lights));

    lights
        .iter()
        .map(|row| row.iter().filter(|&&x| x).count())
        .sum()
}

fn main() {
    println!("{}", part_one());
}
