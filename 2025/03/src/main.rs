use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct BatteryHolder<const N: usize> {
    batteries: [u8; N],
    stored: usize,
}

impl<const N: usize> BatteryHolder<N> {
    fn new() -> BatteryHolder<N> {
        BatteryHolder {
            batteries: [0; N],
            stored: 0,
        }
    }

    fn absorb(&mut self, b: u8) {
        if self.stored < N {
            self.batteries[self.stored] = b;
            self.stored += 1;
        } else {
            let (replace_from, replace_val) = self
                .batteries
                .iter()
                .enumerate()
                .min_by_key(|(_, n)| (*n).clone())
                .unwrap();

            if replace_val > &b {
                if let Some(n) = self
                    .batteries
                    .iter()
                    .tuple_windows::<(_, _)>()
                    .enumerate()
                    .find(|(_, (a, b))| a < b)
                    .map(|(i, _)| i)
                {
                    for i in n..(N - 1) {
                        self.batteries[i] = self.batteries[i + 1];
                    }
                    self.batteries[N - 1] = b;
                }

                return;
            }

            for i in replace_from..(N - 1) {
                self.batteries[i] = self.batteries[i + 1];
            }
            self.batteries[N - 1] = b;
        }
    }
}

impl<const N: usize> IntoIterator for BatteryHolder<N> {
    type Item = u8;
    type IntoIter = core::array::IntoIter<u8, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.batteries.into_iter()
    }
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut holder = BatteryHolder::<2>::new();
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .for_each(|u| holder.absorb(u));
            holder
                .into_iter()
                .map(|b| b.to_string())
                .reduce(|a, b| a + &b)
                .unwrap()
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn main() {
    println!("{}", part_one());
}
