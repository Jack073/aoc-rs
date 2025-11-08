use itertools::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::take;

#[derive(Clone, Debug)]
struct Coordinate {
    coords: [i8; 4],
}

impl Coordinate {
    pub fn distance_to(&self, coord: &Self) -> u8 {
        izip!(self.coords, coord.coords)
            .map(|(a, b)| a - b)
            .map(|n| n.abs())
            .fold(0, |a, b| a + b) as u8
    }

    pub fn new(list: &str) -> Self {
        Self {
            coords: list
                .split(",")
                .map(|x| x.parse::<i8>().unwrap())
                .collect_array()
                .unwrap(),
        }
    }
}

fn part_one() -> usize {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);

    let coord_list = reader
        .lines()
        .filter_map(|n| if let Ok(a) = n { Some(a) } else { None })
        .map(|line| Coordinate::new(&line))
        .collect_vec();

    let mut constellations: Vec<Vec<&Coordinate>> = Vec::with_capacity(coord_list.len());

    constellations.resize_with(coord_list.len(), || Vec::new());

    for (index, coord) in coord_list.iter().enumerate() {
        constellations[index].push(coord);
    }

    let mut has_changed = true;

    while has_changed {
        has_changed = false;
        for outer in 0..constellations.len() {
            if constellations[outer].is_empty() {
                continue;
            }

            for inner in 0..constellations.len() {
                if inner == outer {
                    continue;
                }
                if constellations[inner].is_empty() {
                    continue;
                }

                let compare_vec = &constellations[inner];
                let constellation = &constellations[outer];

                if constellation
                    .iter()
                    .any(|c| compare_vec.iter().any(|c2| c.distance_to(c2) <= 3))
                {
                    has_changed = true;

                    let new_extension = take(&mut constellations[inner]);

                    constellations[outer].extend(new_extension);
                }
            }
        }
    }

    constellations.iter().filter(|c| !c.is_empty()).count()
}

fn main() {
    println!("{}", part_one());
}
