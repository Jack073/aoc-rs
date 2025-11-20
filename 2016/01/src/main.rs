use std::fs::File;
use std::io::Read;

struct InstructionCounter {
    north: usize,
    south: usize,
    east: usize,
    west: usize,
}

impl InstructionCounter {
    fn increment_direction(&mut self, direction: &Direction, delta: usize) {
        match direction {
            Direction::North => self.north += delta,
            Direction::South => self.south += delta,
            Direction::East => self.east += delta,
            Direction::West => self.west += delta,
        }
    }

    fn taxicab_distance(&self) -> usize {
        (self.north.abs_diff(self.south)) + (self.east.abs_diff(self.west))
    }
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            Self::North => Self::West,
            Self::East => Self::North,
            Self::South => Self::East,
            Self::West => Self::South,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
}

fn part_one() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .expect("input file error")
        .read_to_string(&mut s)
        .expect("read error");

    let mut accumulator = InstructionCounter {
        north: 0,
        south: 0,
        east: 0,
        west: 0,
    };

    let mut direction = Direction::North;

    s.split(", ")
        .map(|instruction| {
            let mut chars = instruction.chars();

            let direction = chars.next().unwrap();

            (
                direction,
                chars.collect::<String>().parse::<usize>().unwrap(),
            )
        })
        .for_each(|(d, distance)| {
            direction = match d {
                'L' => direction.left(),
                'R' => direction.right(),
                _ => panic!("invalid direction"),
            };

            accumulator.increment_direction(&direction, distance);
        });

    accumulator.taxicab_distance()
}

fn main() {
    println!("{}", part_one());
}
