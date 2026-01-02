use std::cmp::PartialEq;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{BitAnd, BitOr, BitXor, Sub};

trait AlphabetNumber {
    fn alphabet_number(&self) -> u8;
}

impl AlphabetNumber for char {
    fn alphabet_number(&self) -> u8 {
        (*self as u8) - ('a' as u8)
    }
}

struct Segment {
    inner: u8,
    cnt: usize,
}

impl BitXor for &Segment {
    type Output = Segment;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let inner = self.inner ^ rhs.inner;
        Self::Output {
            inner,
            cnt: inner.count_ones() as usize,
        }
    }
}

impl BitOr for &Segment {
    type Output = Segment;

    fn bitor(self, rhs: Self) -> Self::Output {
        let inner = self.inner | rhs.inner;
        Self::Output {
            inner,
            cnt: inner.count_ones() as usize,
        }
    }
}

impl BitAnd for &Segment {
    type Output = Segment;

    fn bitand(self, rhs: Self) -> Self::Output {
        let inner = self.inner & rhs.inner;
        Self::Output {
            inner,
            cnt: inner.count_ones() as usize,
        }
    }
}

impl Sub for &Segment {
    type Output = Segment;
    fn sub(self, rhs: Self) -> Self::Output {
        let inner = self.inner - rhs.inner;
        Self::Output {
            inner,
            cnt: inner.count_ones() as usize,
        }
    }
}

impl From<String> for Segment {
    fn from(s: String) -> Self {
        let inner = s
            .chars()
            .into_iter()
            .map(|c| 1 << c.alphabet_number())
            .fold(0, |a, b| a ^ b);

        Self {
            inner,
            cnt: inner.count_ones() as usize,
        }
    }
}

impl From<char> for Segment {
    fn from(c: char) -> Self {
        Self {
            inner: 1 << c.alphabet_number(),
            cnt: 1,
        }
    }
}

impl Segment {
    fn extract(&self) -> char {
        Self::digit_to_char((0..8).find(|i| (self.inner) & (1u8 << i) != 0u8).unwrap())
    }

    fn matrix(&self) -> [u8; 7] {
        [
            self.inner & 1,
            (self.inner & 2) >> 1,
            (self.inner & 4) >> 2,
            (self.inner & 8) >> 3,
            (self.inner & 16) >> 4,
            (self.inner & 32) >> 5,
            (self.inner & 64) >> 6,
        ]
    }

    fn digit_to_char(d: u8) -> char {
        ((d + 1) | 96u8) as char
    }

    fn missing(&self) -> Self {
        let inner = !self.inner;
        Self {
            inner,
            cnt: inner.count_ones() as usize,
        }
    }
}

impl PartialEq<&Segment> for Segment {
    fn eq(&self, other: &&Segment) -> bool {
        self.inner == other.inner
    }
}

fn build_dict(sample: Vec<String>, dict: &mut [char; 7]) {
    /*
       0 - 6 (a,b,c, ,e,f,g) missing       d
       1 - 2 ( , ,c, , ,f, ) missing a b   d e    g
       2 - 5 (a, ,c,d,e, ,g) missing   b       f
       3 - 5 (a, ,c,d, ,f,g) missing   b     e
       4 - 4 ( ,b,c,d, ,f, ) missing a       e    g
       5 - 5 (a,b, ,d, ,f,g) missing     c   e
       6 - 6 (a,b, ,d,e,f,g) missing     c
       7 - 3 (a, ,c, , ,f, ) missing   b   d e    g
       8 - 7 (a,b,c,d,e,f,g) missing
       9 - 6 (a,b,c,d, ,f,g) missing         e
    */

    let segments = sample
        .into_iter()
        .map(|s| Segment::from(s))
        .collect::<Vec<_>>();

    // Calculate 'a'
    {
        let one = segments.iter().find(|s| s.cnt == 2).unwrap();
        let seven = segments.iter().find(|s| s.cnt == 3).unwrap();

        dict[0] = (one ^ seven).extract();

        // Calculate letters from missing letter matrix (b, e, f)

        let summed_matrix = segments
            .iter()
            .map(|s| s.matrix())
            .reduce(|mut a, b| {
                for (i, val) in b.iter().enumerate() {
                    a[i] += *val;
                }
                a
            })
            .unwrap();

        summed_matrix.iter().enumerate().for_each(|(i, val)| {
            match val {
                9 => {
                    // f
                    dict[5] = Segment::digit_to_char(i as u8);
                }
                4 => {
                    // e
                    dict[4] = Segment::digit_to_char(i as u8);
                }
                6 => {
                    // b
                    dict[1] = Segment::digit_to_char(i as u8);
                }
                _ => {}
            };
        });

        // Calculate 'c'
        dict[2] = (one - &dict[5].into()).extract();
    }

    // Calculate 'd'
    {
        let c = &dict[2].into();
        let e = &dict[4].into();

        dict[3] = segments
            .iter()
            .find(|&s| s.cnt == 6 && (s & c) == c && (s & e) == e)
            .unwrap()
            .missing()
            .extract();
    }

    // Finally calculate 'g'
    {
        dict[6] = (0..6)
            .map(|c| Segment::from(dict[c]))
            .reduce(|a, b| &a | &b)
            .unwrap()
            .missing()
            .extract();
    }
}

// This could easily have also been a [u8;128].
fn calculate_digit(val: u8) -> u8 {
    const ZERO: u8 = 119;
    const ONE: u8 = 36;
    const TWO: u8 = 93;
    const THREE: u8 = 109;
    const FOUR: u8 = 46;
    const FIVE: u8 = 107;
    const SIX: u8 = 123;
    const SEVEN: u8 = 37;
    const EIGHT: u8 = 127;
    const NINE: u8 = 111;

    match val {
        ZERO => 0,
        ONE => 1,
        TWO => 2,
        THREE => 3,
        FOUR => 4,
        FIVE => 5,
        SIX => 6,
        SEVEN => 7,
        EIGHT => 8,
        NINE => 9,
        _ => panic!("invalid digit value: {}", val),
    }
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (raw_samples, display) = l.split_once(" | ").unwrap();

            let samples = raw_samples
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();
            let mut dict = ['0'; 7];

            build_dict(samples, &mut dict);

            display
                .split_ascii_whitespace()
                .map(|digit| {
                    Segment::from(
                        digit
                            .chars()
                            .map(|c| {
                                Segment::digit_to_char(
                                    dict.iter().position(|&d| c == d).unwrap() as u8
                                )
                            })
                            .collect::<String>(),
                    )
                })
                .map(|s| calculate_digit(s.inner))
                .filter(|&d| match d {
                    1 | 4 | 7 | 8 => true,
                    _ => false,
                })
                .count()
        })
        .sum::<usize>()
}

fn part_two() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (raw_samples, display) = l.split_once(" | ").unwrap();

            let samples = raw_samples
                .split_ascii_whitespace()
                .map(|s| s.to_owned())
                .collect::<Vec<_>>();
            let mut dict = ['0'; 7];

            build_dict(samples, &mut dict);

            display
                .split_ascii_whitespace()
                .map(|digit| {
                    Segment::from(
                        digit
                            .chars()
                            .map(|c| {
                                Segment::digit_to_char(
                                    dict.iter().position(|&d| c == d).unwrap() as u8
                                )
                            })
                            .collect::<String>(),
                    )
                })
                .map(|s| calculate_digit(s.inner) as usize)
                .reduce(|a, b| (a * 10) + b)
                .unwrap()
        })
        .sum::<usize>()
}

fn main() {
    println!("{}", part_two());
}
