use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

struct Range {
    start: usize,
    end: usize,
    obsolete: bool,
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
            obsolete: false,
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

fn part_two() -> usize {
    let mut reader = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|n| n.unwrap());

    let mut ranges = reader
        .take_while(|line| line.len() > 0)
        .map(|line| {
            line.split_terminator('-')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Range>()
        })
        .collect::<Vec<_>>();

    let len = ranges.len();

    for mut r1 in 0..len {
        if ranges[r1].obsolete {
            continue;
        }

        for mut r2 in 0..len {
            if r1 == r2 {
                continue;
            }

            if ranges[r2].obsolete {
                continue;
            }

            let mut obsolete = false;
            let mut extend = false;
            {
                let range_1 = &ranges[r1];
                let range_2 = &ranges[r2];

                if range_1.start <= range_2.start && range_2.start <= range_1.end {
                    if range_2.end > range_1.end {
                        extend = true;
                    } else {
                        obsolete = true;
                    }
                } else {
                    if range_2.start <= range_1.start && range_1.start <= range_2.end {
                        if range_1.end > range_2.end {
                            extend = true;
                        } else {
                            obsolete = true;
                        }

                        swap(&mut r1, &mut r2);
                    }
                }
            }

            if extend {
                ranges[r1].end = ranges[r2].end;
                ranges[r2].obsolete = true;
            }

            if obsolete {
                ranges[r2].obsolete = true;
            }
        }
    }

    ranges
        .iter()
        .filter(|r| !r.obsolete)
        .map(|r| 1 + r.end - r.start)
        .sum()
}

fn main() {
    println!("{}", part_two());
}
