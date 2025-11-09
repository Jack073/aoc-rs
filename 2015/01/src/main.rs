use std::fs::File;
use std::io::Read;

fn get_sequence() -> impl Iterator<Item = isize> {
    let mut s = String::new();
    File::open("input.txt")
        .expect("input file")
        .read_to_string(&mut s)
        .expect("read error");

    s.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("invalid number"),
        })
        .collect::<Vec<isize>>()
        .into_iter()
}

fn part_one() -> isize {
    get_sequence().sum()
}

fn part_two() -> isize {
    get_sequence()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .position(|n| n == -1)
        .unwrap() as isize
        + 1
}

fn main() {
    println!("{}", part_two());
}
