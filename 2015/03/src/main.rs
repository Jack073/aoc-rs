use std::fs::File;
use std::io::Read;

use itertools::Itertools;

fn part_one() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .expect("input file not found")
        .read_to_string(&mut s)
        .expect("read error");

    let mut coords = s
        .chars()
        .map(|c| match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (-1, 0),
            '<' => (1, 0),
            _ => panic!("bad character"),
        })
        .scan((0, 0), |acc, coord| {
            acc.0 += coord.0;
            acc.1 += coord.1;

            Some(*acc)
        })
        .collect::<Vec<_>>();
    coords.push((0, 0));
    coords.sort_by_key(|n| n.0.to_string() + ">" + &n.1.to_string());

    coords
        .iter()
        .chunk_by(|&a| a)
        .into_iter()
        .filter_map(|n| if n.1.count() >= 1 { Some(1) } else { None })
        .count()
}

fn part_two() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .expect("input file not found")
        .read_to_string(&mut s)
        .expect("read error");

    let mut coords = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if i % 2 == 0 { Some(c) } else { None })
        .map(|c| match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (-1, 0),
            '<' => (1, 0),
            _ => panic!("bad character"),
        })
        .scan((0, 0), |acc, coord| {
            acc.0 += coord.0;
            acc.1 += coord.1;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    let robo_coords = s
        .chars()
        .enumerate()
        .filter_map(|(i, c)| if i % 2 == 1 { Some(c) } else { None })
        .map(|c| match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (-1, 0),
            '<' => (1, 0),
            _ => panic!("bad character"),
        })
        .scan((0, 0), |acc, coord| {
            acc.0 += coord.0;
            acc.1 += coord.1;
            Some(*acc)
        })
        .collect::<Vec<_>>();

    coords.extend(robo_coords);

    coords.push((0, 0));
    coords.push((0, 0));

    coords.sort_by_key(|n| n.0.to_string() + ">" + &n.1.to_string());

    coords
        .iter()
        .chunk_by(|&a| a)
        .into_iter()
        .filter_map(|n| if n.1.count() >= 1 { Some(1) } else { None })
        .count()
}

fn main() {
    println!("{}", part_two());
}
