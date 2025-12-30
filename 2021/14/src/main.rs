use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

use itertools::Itertools;

trait Key
where
    Self: Sized,
{
    fn key(&self) -> usize;

    fn countable_character(&self) -> usize;

    fn split_with(&self, splitter: usize) -> (Self, Self);
}

#[derive(Copy, Clone)]
struct Pair {
    left: usize,
    right: usize,
    count: usize,
}

impl From<usize> for Pair {
    fn from(value: usize) -> Self {
        Self {
            left: value & 0x1F,
            right: value >> 5,
            count: 0,
        }
    }
}

impl From<(char, char)> for Pair {
    fn from(value: (char, char)) -> Self {
        Self {
            left: value.0 as usize - ('A' as usize),
            right: value.1 as usize - ('A' as usize),
            count: 0,
        }
    }
}

impl AddAssign<Pair> for Option<Pair> {
    fn add_assign(&mut self, rhs: Pair) {
        match self {
            None => {
                *self = Some(rhs);
            }
            Some(s) => {
                s.count += rhs.count;
            }
        }
    }
}

impl Key for Pair {
    fn key(&self) -> usize {
        (self.right << 5) | self.left
    }

    fn countable_character(&self) -> usize {
        self.right
    }

    fn split_with(&self, splitter: usize) -> (Self, Self) {
        (
            Pair {
                left: self.left,
                right: splitter,
                count: self.count,
            },
            Pair {
                left: splitter,
                right: self.right,
                count: self.count,
            },
        )
    }
}

fn react(counts: &mut [Option<Pair>; 1024], rules: [Option<usize>; 1024], steps: usize) {
    for _ in 0..steps {
        let mut counts_clone = [None; 1024];
        for index in 0..counts.len() {
            if let Some(count) = counts[index] {
                match rules[index] {
                    Some(rule) => {
                        let (l, r) = count.split_with(rule);
                        counts_clone[l.key()] += l;
                        counts_clone[r.key()] += r;
                    }
                    None => {
                        counts_clone[index] = counts[index];
                    }
                };
            }
        }

        *counts = counts_clone;
    }
}

fn process_reactions(rounds: usize) -> usize {
    let mut lines = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap());

    let mut state: [Option<Pair>; 1024] = [None; 1024];

    let starting_line = lines.next().unwrap();

    starting_line
        .chars()
        .into_iter()
        .tuple_windows::<(_, _)>()
        .for_each(|window| {
            let mut p = Pair::from(window);
            p.count = 1;
            state[p.key()] += p;
        });
    lines.next();

    let mut rules: [Option<usize>; 1024] = [None; 1024];

    lines.for_each(|line| {
        let chars = line.chars().collect::<Vec<char>>();
        let key = Pair::from((chars[0], chars[1])).key();

        let splitter = chars[6] as usize - ('A' as usize);

        rules[key] = Some(splitter);
    });

    react(&mut state, rules, rounds);

    let mut frequencies = [0; 26];

    frequencies[starting_line.chars().next().unwrap() as usize - ('A' as usize)] = 1;

    state.iter().for_each(|value| match value {
        None => {}
        Some(s) => {
            frequencies[s.countable_character()] += s.count;
        }
    });

    frequencies.iter().max().unwrap() - frequencies.iter().filter(|&p| *p > 0).min().unwrap()
}

fn part_one() -> usize {
    process_reactions(10)
}

fn part_two() -> usize {
    process_reactions(40)
}

fn main() {
    println!("{}", part_two());
}
