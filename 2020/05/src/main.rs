use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_seats() -> impl Iterator<Item = i32> {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut low_row = 0;
            let mut high_row = 127;

            let mut low_col = 0;
            let mut high_col = 7;

            line.chars().for_each(|c| {
                match c {
                    'F' => {
                        high_row = (low_row + high_row) / 2;
                    }
                    'B' => {
                        low_row = 1 + ((high_row + low_row) / 2);
                    }
                    'L' => {
                        high_col = (low_col + high_col) / 2;
                    }
                    'R' => {
                        low_col = 1 + ((high_col + low_col) / 2);
                    }
                    _ => {}
                };
            });

            (low_row * 8) + low_col
        })
}

fn part_one() -> i32 {
    get_seats().max().unwrap()
}

fn part_two() -> i32 {
    let mut seats = get_seats().collect::<Vec<_>>();

    seats
        .iter()
        .find(|&s1| (!seats.contains(&(s1 + 1))) && seats.contains(&(s1 + 2)))
        .unwrap()
        + 1
}

fn main() {
    println!("{}", part_two());
}
