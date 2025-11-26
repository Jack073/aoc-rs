use std::fs::File;
use std::io::{BufRead, BufReader};

enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn play(&self, other: &Self) -> usize {
        match self {
            Self::Rock => match other {
                Self::Rock => 3,
                Self::Paper => 0,
                Self::Scissors => 6,
            },
            Self::Paper => match other {
                Self::Rock => 6,
                Self::Paper => 3,
                Self::Scissors => 0,
            },
            Self::Scissors => match other {
                Self::Rock => 0,
                Self::Paper => 6,
                Self::Scissors => 3,
            },
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn from_letter(letter: char) -> Self {
        match letter {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => panic!("unknown letter"),
        }
    }
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").expect("input.txt error"))
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            (
                RPS::from_letter(l.chars().nth(0).unwrap()),
                RPS::from_letter(l.chars().nth(2).unwrap()),
            )
        })
        .map(|(r1, r2)| r2.play(&r1) + r2.value())
        .sum::<usize>()
}

fn main() {
    println!("{}", part_one());
}
