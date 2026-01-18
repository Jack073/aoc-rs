use std::fs::File;
use std::io::{BufRead, BufReader};

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let split = line.split_at(line.len() / 2);

            [split.0, split.1]
                .map(|compartment| {
                    compartment.chars().fold(0u64, |acc, c| {
                        if c.is_uppercase() {
                            acc | (1 << (c as u64 - 'A' as u64) + 27)
                        } else {
                            acc | (1 << (c as u64 - 'a' as u64) + 1)
                        }
                    })
                })
                .into_iter()
                .reduce(|acc, new| acc & new)
                .unwrap()
                .trailing_zeros() as usize
        })
        .sum()
}

// This would be simpler to import itertools, but for one small function it felt like a better
// opportunity to just implement it myself rather than add a dependency.
trait Chunker<const N: usize, I: Iterator> {
    type Item;

    fn chunks(self) -> Chunked<N, I>;
}

struct Chunked<const N: usize, I: Iterator> {
    src: I,
}

impl<const N: usize, T: Iterator> Iterator for Chunked<N, T>
where
    T::Item: Default + Copy,
{
    type Item = [T::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf: Self::Item = [Default::default(); N];
        for i in 0..N {
            match self.src.next() {
                Some(p) => buf[i] = p,
                None => return None,
            }
        }

        Some(buf)
    }
}

impl<I: Iterator + Sized, const N: usize> Chunker<N, I> for I {
    type Item = I::Item;

    fn chunks(self) -> Chunked<N, I> {
        Chunked { src: self }
    }
}

fn part_two() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars().fold(0u64, |acc, c| {
                if c.is_uppercase() {
                    acc | (1 << (c as u64 - 'A' as u64) + 27)
                } else {
                    acc | (1 << (c as u64 - 'a' as u64) + 1)
                }
            })
        })
        .chunks()
        .map(|chunk: [u64; 3]| {
            chunk
                .into_iter()
                .reduce(|acc, new| acc & new)
                .unwrap()
                .trailing_zeros() as usize
        })
        .sum()
}


fn main() {
    println!("{}", part_two());
}
