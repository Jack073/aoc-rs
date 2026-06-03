use std::fs::File;
use std::io::{BufRead, BufReader};

fn gather_fish() -> [usize; 9] {
    BufReader::new(File::open("input.txt").unwrap())
        .split(b',')
        .map(|n| n.unwrap())
        .map(String::from_utf8)
        .map(Result::unwrap)
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut acc, val| {
            acc[val] += 1;
            acc
        })
}

fn simulate_growth(mut state: [usize; 9], days: usize) -> usize {
    for _ in 0..days {
        let new_fish = state[0];
        for n in 1..=8 {
            state[n - 1] = state[n];
        }
        state[6] += new_fish;
        state[8] = new_fish;
    }

    state.iter().sum()
}

fn part_one() -> usize {
    let fish = gather_fish();
    simulate_growth(fish, 80)
}

fn part_two() -> usize {
    let fish = gather_fish();
    simulate_growth(fish, 256)
}

fn main() {
    println!("{}", part_two());
}
