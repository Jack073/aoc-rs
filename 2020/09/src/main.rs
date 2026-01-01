use std::fs::File;
use std::io::{BufRead, BufReader};

struct StatefulIterator<T, const N: usize>
where
    T: Iterator<Item = usize>,
{
    src: T,
    buf: [usize; N],
    pos: usize,
    init: bool,
}

impl<T, const N: usize> Iterator for StatefulIterator<T, N>
where
    T: Iterator<Item = usize>,
{
    type Item = (usize, bool);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.init {
            self.src.by_ref().take(N).enumerate().for_each(|(i, v)| {
                self.buf[i] = v;
            });
            self.init = true;
        }

        let next_val = self.src.next()?;

        let possible = self
            .buf
            .iter()
            .any(|i1| self.buf.iter().any(|i2| i1 + i2 == next_val));

        self.buf[self.pos] = next_val;
        self.pos += 1;
        if self.pos == N {
            self.pos = 0;
        }

        Some((next_val, possible))
    }
}

fn part_one() -> usize {
    StatefulIterator {
        src: BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap().parse().unwrap()),
        buf: [0; 25],
        pos: 0,
        init: false,
    }
    .into_iter()
    .find(|(_, possible)| !possible)
    .unwrap()
    .0
}

fn main() {
    println!("{}", part_one());
}
