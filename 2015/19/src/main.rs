use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;

#[derive(Clone)]
struct SplitInput<'a> {
    before: &'a str,
    changeable_section: &'a str,
    substituted: Option<&'a str>,
    after: &'a str,
}

impl<'a> SplitInput<'a> {
    fn to_string(&self) -> String {
        let mut s = String::new();
        s.push_str(self.before);
        match self.substituted {
            None => s.push_str(self.changeable_section),
            Some(n) => s.push_str(n),
        };
        s.push_str(self.after);
        s
    }

    fn create(src: &'a str, substitutable_range: Range<usize>) -> Self {
        Self {
            before: &src[0..substitutable_range.start],
            after: &src[substitutable_range.end..],
            changeable_section: &src[substitutable_range],
            substituted: None,
        }
    }

    fn apply_all(&mut self, changes: &Vec<(&'a String, &'a String)>) -> Vec<String> {
        changes
            .into_iter()
            .filter_map(|change| {
                if change.0 == self.changeable_section {
                    self.substituted = Some(change.1);
                    Some(self.to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }
}

fn part_one() -> usize {
    let mut lines = BufReader::new(File::open("input.txt").expect("input read error")).lines();

    let rules = lines
        .by_ref()
        .take_while(|line| match line {
            Ok(n) => n != "",
            Err(_) => panic!("err reading file"),
        })
        .map(|line| {
            line.unwrap()
                .split_terminator(" => ")
                .map(|n| n.to_string())
                .collect_tuple::<(_, _)>()
                .expect("tuple error")
        })
        .collect::<Vec<_>>();

    let mut organised_rules = Vec::new();
    for rule in rules.iter() {
        organised_rules.push((&rule.0, &rule.1));
    }

    let input = lines.next().unwrap().unwrap();
    let len = input.len();
    (0..len)
        .flat_map(|i| {
            vec![
                SplitInput::create(&input, i..usize::min(i + 1, len)),
                SplitInput::create(&input, i..usize::min(len, i + 2)),
            ]
            .into_iter()
            .map(|mut s| s.apply_all(&organised_rules))
            .filter(|x| !x.is_empty())
            .collect_vec()
        })
        .flatten()
        .unique()
        .count()
}

fn main() {
    println!("{}", part_one());
}
