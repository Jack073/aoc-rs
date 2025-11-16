use std::fs::File;
use std::io::Read;

use itertools::izip;

fn part_one() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .expect("input file error")
        .read_to_string(&mut s)
        .expect("read error");

    let mut chars_one = s.chars().collect::<Vec<_>>();
    let chars_two = chars_one.clone();
    chars_one.push(chars_one[0]);
    chars_one.remove(0);

    izip!(chars_one, chars_two)
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a as usize - '0' as usize)
        .sum()
}

fn part_two() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .expect("input file error")
        .read_to_string(&mut s)
        .expect("read error");

    let mut chars_one = s.chars().collect::<Vec<_>>();
    let chars_two = chars_one.clone();

    let halfway = chars_one.len() / 2;

    chars_one = [&chars_two[halfway..], &chars_two[..halfway]].concat();

    izip!(chars_one, chars_two)
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a as usize - '0' as usize)
        .sum()
}

fn main() {
    println!("{}", part_two());
}
