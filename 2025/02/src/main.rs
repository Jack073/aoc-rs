use std::fs::File;
use std::io::{BufRead, Read};

fn part_one() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut s)
        .unwrap();

    s.split(",")
        .map(|range| {
            let mut split_range = range
                .split_terminator("-")
                .map(str::parse::<usize>)
                .map(Result::unwrap);
            let start = split_range.next().unwrap();
            let end = split_range.next().unwrap();

            (start..=end)
                .filter(|num| {
                    let s = num.to_string();

                    if s.len() % 2 == 1 {
                        false
                    } else {
                        let chars = s.chars().collect::<Vec<char>>();
                        let midpoint = chars.len() / 2;
                        (0..midpoint).all(|c| chars[c] == chars[c + midpoint])
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    println!("{}", part_one());
}
