use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line {
    target: u16,
    options: Vec<u16>,
    joltage: Vec<u16>,

    max_combinations: u16,
    counter: u16,
}

impl From<String> for Line {
    fn from(value: String) -> Self {
        let split = value.split_ascii_whitespace().collect::<Vec<_>>();

        Self {
            target: split[0]
                .trim_start_matches("[")
                .trim_end_matches("]")
                .chars()
                .enumerate()
                .filter(|(i, enabled)| enabled == &'#')
                .map(|(i, _)| i)
                .fold(0, |a, b| a | (1 << b)),
            options: (1..(split.len() - 1))
                .map(|i| split[i])
                .map(|opt| {
                    opt.trim_start_matches("(")
                        .trim_end_matches(")")
                        .split(",")
                        .map(|n| n.parse::<u16>().unwrap())
                        .fold(0, |a, b| a | (1 << b))
                })
                .collect::<Vec<_>>(),
            joltage: split[split.len() - 1]
                .trim_start_matches("{")
                .trim_end_matches("}")
                .split(",")
                .map(|n| n.parse::<u16>().unwrap())
                .collect(),
            max_combinations: 1 << (split.len() - 2),
            counter: 0,
        }
    }
}

impl Iterator for Line {
    type Item = Option<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.max_combinations <= self.counter {
            return None;
        }

        let mut state = 0;

        let mut set = 0;

        for n in 0..=15 {
            if self.counter & (1 << n) > 0 {
                set += 1;

                state ^= self.options[n];
            }
        }

        self.counter += 1;

        if state == self.target {
            Some(Some(set))
        } else {
            Some(None)
        }
    }
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("missing input file"))
        .lines()
        .map(|l| l.unwrap())
        .map(Line::from)
        .map(|l| l.into_iter().filter_map(|l| l).min().unwrap())
        .sum::<usize>()
}

fn main() {
    println!("{}", part_one());
}
