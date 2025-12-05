use std::fs::File;
use std::io::{BufRead, BufReader};

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, n: usize) -> bool {
        self.start <= n && n <= self.end
    }
}

impl FromIterator<usize> for Range {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        Self {
            start: it.next().unwrap(),
            end: it.last().unwrap(),
        }
    }
}

fn part_one() -> usize {
    let mut reader = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|n| n.unwrap());

    let ranges = reader
        .by_ref()
        .take_while(|line| line.len() > 0)
        .map(|line| {
            line.split_terminator('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Range>()
        })
        .collect::<Vec<_>>();

    reader
        .map(|l| l.parse::<usize>().unwrap())
        .filter(|line| ranges.iter().any(|r| r.contains(*line)))
        .count()
}

fn main() {
    println!("{}", part_one());
}
