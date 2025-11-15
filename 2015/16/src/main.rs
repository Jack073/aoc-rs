use std::fs::File;
use std::io::{BufRead, BufReader};

struct Sue {
    id: isize,

    children: Option<isize>,
    cats: Option<isize>,
    samoyeds: Option<isize>,
    pomeranians: Option<isize>,
    akitas: Option<isize>,
    vizslas: Option<isize>,
    goldfish: Option<isize>,
    trees: Option<isize>,
    cars: Option<isize>,
    perfumes: Option<isize>,
}

impl Sue {
    fn matches(&self, cmp: &Sue) -> bool {
        [
            cmp.children
                .map(|children| children == self.children.unwrap()),
            cmp.cats.map(|cats| cats == self.cats.unwrap()),
            cmp.samoyeds
                .map(|samoyeds| samoyeds == self.samoyeds.unwrap()),
            cmp.pomeranians
                .map(|pomeranians| pomeranians == self.pomeranians.unwrap()),
            cmp.akitas.map(|akitas| akitas == self.akitas.unwrap()),
            cmp.vizslas.map(|vizslas| vizslas == self.vizslas.unwrap()),
            cmp.goldfish
                .map(|goldfish| goldfish == self.goldfish.unwrap()),
            cmp.trees.map(|trees| trees == self.trees.unwrap()),
            cmp.cars.map(|cars| cars == self.cars.unwrap()),
            cmp.perfumes
                .map(|perfumes| perfumes == self.perfumes.unwrap()),
        ]
        .into_iter()
        .filter_map(|n| n)
        .all(|v| v)
    }

    fn matches_part_two(&self, cmp: &Sue) -> bool {
        [
            cmp.children
                .map(|children| children == self.children.unwrap()),
            cmp.cats.map(|cats| cats > self.cats.unwrap()),
            cmp.samoyeds
                .map(|samoyeds| samoyeds == self.samoyeds.unwrap()),
            cmp.pomeranians
                .map(|pomeranians| pomeranians < self.pomeranians.unwrap()),
            cmp.akitas.map(|akitas| akitas == self.akitas.unwrap()),
            cmp.vizslas.map(|vizslas| vizslas == self.vizslas.unwrap()),
            cmp.goldfish
                .map(|goldfish| goldfish < self.goldfish.unwrap()),
            cmp.trees.map(|trees| trees > self.trees.unwrap()),
            cmp.cars.map(|cars| cars == self.cars.unwrap()),
            cmp.perfumes
                .map(|perfumes| perfumes == self.perfumes.unwrap()),
        ]
            .into_iter()
            .filter_map(|n| n)
            .all(|v| v)
    }

    fn construct(line: String) -> Self {
        let mut split = line.split_ascii_whitespace();

        let mut sue = Sue {
            id: Self::parse_number(&mut split).unwrap().1,
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        };

        loop {
            match Self::parse_number(&mut split) {
                Some(("children:", n)) => {
                    sue.children = Some(n);
                }
                Some(("cats:", n)) => {
                    sue.cats = Some(n);
                }
                Some(("samoyeds:", n)) => {
                    sue.samoyeds = Some(n);
                }
                Some(("pomeranians:", n)) => {
                    sue.pomeranians = Some(n);
                }
                Some(("akitas:", n)) => {
                    sue.akitas = Some(n);
                }
                Some(("vizslas:", n)) => {
                    sue.vizslas = Some(n);
                }
                Some(("goldfish:", n)) => {
                    sue.goldfish = Some(n);
                }
                Some(("trees:", n)) => {
                    sue.trees = Some(n);
                }
                Some(("cars:", n)) => {
                    sue.cars = Some(n);
                }
                Some(("perfumes:", n)) => {
                    sue.perfumes = Some(n);
                }
                _ => {
                    break;
                }
            }
        }

        sue
    }

    fn parse_number<'a>(src: &mut impl Iterator<Item = &'a str>) -> Option<(&'a str, isize)> {
        let name = src.next();
        match name {
            Some(n) => Some((
                n,
                src.next()
                    .unwrap()
                    .trim_end_matches(":")
                    .trim_end_matches(",")
                    .parse::<_>()
                    .unwrap(),
            )),
            None => None,
        }
    }
}

fn part_one() -> isize {
    let target_sue = Sue {
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    BufReader::new(File::open("input.txt").expect("file open error"))
        .lines()
        .filter_map(|n| match n {
            Ok(a) => Some(a),
            Err(_) => return None,
        })
        .map(Sue::construct)
        .find(|sue| target_sue.matches(&sue))
        .expect("missing sue")
        .id
}

fn part_two() -> isize {
    let target_sue = Sue {
        id: 0,
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: Some(0),
        vizslas: Some(0),
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    BufReader::new(File::open("input.txt").expect("file open error"))
        .lines()
        .filter_map(|n| match n {
            Ok(a) => Some(a),
            Err(_) => return None,
        })
        .map(Sue::construct)
        .find(|sue| target_sue.matches_part_two(&sue))
        .expect("missing sue")
        .id
}


fn main() {
    println!("{}", part_two());
}
