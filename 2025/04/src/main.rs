use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    let map = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '@' => true,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let deltas = [
        (1isize, 0isize),
        (1, 1),
        (1, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
    ];

    (0..map.len())
        .map(|x| {
            (0..map[x].len())
                .filter(|y| map[x][*y])
                .map(|y| {
                    deltas
                        .iter()
                        .map(|delta| {
                            if delta.0 < 0 && x == 0 {
                                return false;
                            }

                            if delta.1 < 0 && y == 0 {
                                return false;
                            }

                            if let Some(r) = map.get((delta.0 + x as isize) as usize) {
                                if let Some(v) = r.get((delta.1 + y as isize) as usize) {
                                    return v.clone();
                                }
                            }
                            false
                        })
                        .filter(|n| *n)
                        .count()
                })
                .filter(|adjacent| adjacent < &4)
                .count()
        })
        .sum()
}

fn main() {
    println!("{}", part_one());
}
