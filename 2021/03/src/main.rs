use std::fs::File;
use std::io::{BufRead, BufReader};

struct Combiner {
    counts: Option<Vec<usize>>,
    processed_count: usize,
}

impl Combiner {
    fn new() -> Combiner {
        Self {
            counts: None,
            processed_count: 0,
        }
    }

    fn process_line(&mut self, line: &String) {
        if let None = self.counts {
            let mut v = Vec::new();
            v.resize(line.len(), 0);

            self.counts = Some(v);
        }

        if let Some(counts) = &mut self.counts {
            line.chars().enumerate().for_each(|(i, c)| {
                if c == '1' {
                    counts[i] += 1;
                }
            });
            self.processed_count += 1;
        }
    }

    fn calculate(&self) -> Option<usize> {
        let midpoint = self.processed_count / 2;
        if let Some(counts) = &self.counts {
            let len = counts.len() - 1;
            let res = counts
                .iter()
                .map(|d| if *d >= midpoint { 1 } else { 0 })
                .enumerate()
                .fold(0, |acc, (i, c)| acc | (c << (len - i)));
            Some(res)
        } else {
            None
        }
    }

    fn mask(&self) -> Option<usize> {
        self.counts.as_ref().map(|counts| (1 << counts.len()) - 1)
    }
}

fn part_one() -> usize {
    let mut comb = Combiner::new();
    BufReader::new(File::open("input.txt").expect("input.txt error"))
        .lines()
        .filter_map(Result::ok)
        .for_each(|c| comb.process_line(&c));

    let result = comb.calculate().unwrap();
    result * ((!result) & comb.mask().unwrap())
}

fn main() {
    println!("{}", part_one());
}
