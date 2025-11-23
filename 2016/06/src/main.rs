use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> String {
    let mut frequencies = [[0; 26]; 8];

    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .for_each(|line| {
            line.chars().enumerate().into_iter().for_each(|(i, c)| {
                let index = (c as usize) - ('a' as usize);

                frequencies[i][index] += 1;
            })
        });

    frequencies
        .map(|c| (c.iter().enumerate().max_by_key(|l| l.1).unwrap().0) + 'a' as usize)
        .map(|c| c as u8 as char)
        .iter()
        .collect::<String>()
}


fn part_two() -> String {
    let mut frequencies = [[0; 26]; 8];

    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .for_each(|line| {
            line.chars().enumerate().into_iter().for_each(|(i, c)| {
                let index = (c as usize) - ('a' as usize);

                frequencies[i][index] += 1;
            })
        });

    frequencies
        .map(|c| (c.iter().enumerate().min_by_key(|l| l.1).unwrap().0) + 'a' as usize)
        .map(|c| c as u8 as char)
        .iter()
        .collect::<String>()
}


fn main() {
    println!("{}", part_two());
}
