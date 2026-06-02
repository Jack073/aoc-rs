use std::fs::File;
use std::io::{BufRead, BufReader};

struct WindowedIterator<T, const N: usize>
where
    T::Item: Clone,
    T: Iterator,
{
    src: T,
    buf: [T::Item; N],
    finished: bool,
}

impl<T, const N: usize> Iterator for WindowedIterator<T, N>
where
    T: Iterator,
    T::Item: Clone,
{
    type Item = [T::Item; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let res = self.buf.clone();

        self.buf.rotate_left(1);
        match self.src.next() {
            Some(x) => {
                self.buf[N - 1] = x;
            }
            None => {
                self.finished = true;
            }
        }

        Some(res)
    }
}

trait StreamingWindows<T>
where
    T: Iterator,
    T::Item: Clone,
{
    fn windows<const N: usize>(self) -> Result<WindowedIterator<T, N>, &'static str>;
}

impl<T> StreamingWindows<T> for T
where
    T: Iterator,
    T::Item: Clone + Copy + Default,
{
    fn windows<const N: usize>(mut self) -> Result<WindowedIterator<T, N>, &'static str> {
        let mut buf = [T::Item::default(); N];

        for i in 0..N {
            buf[i] = self.next().ok_or("iterator ended too early")?;
        }

        Ok(WindowedIterator {
            src: self,
            buf,
            finished: false,
        })
    }
}

fn part_one() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap())
        .windows::<2>()
        .expect("iterator too short")
        .map(|[a, b]| a - b)
        .filter(|n| *n < 0)
        .count()
}

fn part_two() -> usize {
    BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().parse::<isize>().unwrap())
        .windows::<3>()
        .unwrap()
        .map(|window| window.iter().sum::<isize>())
        .windows::<2>()
        .expect("iterator too short")
        .map(|[a, b]| a - b)
        .filter(|n| *n < 0)
        .count()
}

fn main() {
    println!("{}", part_two());
}
