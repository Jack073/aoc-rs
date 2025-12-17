use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn abba_check(window: (char, char, char, char)) -> bool {
    window.0 != window.1 && window.0 == window.3 && window.1 == window.2
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|x| {
            let mut has_abba = false;

            for (i, split) in x.rsplit(|c| c == '[' || c == ']').enumerate() {
                if split.chars().tuple_windows().any(abba_check) {
                    if i % 2 == 0 {
                        has_abba = true;
                    } else {
                        return false;
                    }
                }
            }

            has_abba
        })
        .count()
}


fn aba_check(window: &(char, char, char)) -> bool {
    window.0 == window.2 && window.0 != window.1
}

fn construct_aba_vec<'a, T>(iter: T) -> Vec<(char, char, char)>
where
    T: Iterator<Item = &'a &'a str>,
{
    iter.map(|&n| {
        n.chars()
            .tuple_windows()
            .filter(aba_check)
            .collect::<Vec<_>>()
    })
        .flatten()
        .collect::<Vec<_>>()
}

fn part_two() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .filter(|x| {
            let split = x.rsplit(|c| c == '[' || c == ']').collect::<Vec<&str>>();

            let supernet = split.iter().step_by(2);
            let hypernet = split.iter().skip(1).step_by(2);

            let super_matches = construct_aba_vec(supernet);
            let hyper_matches = construct_aba_vec(hypernet);

            super_matches
                .iter()
                .any(|sup| hyper_matches.iter().any(|m| sup.0 == m.1 && sup.1 == m.0))
        })
        .count()
}

fn main() {
    println!("{}", part_one());
}
