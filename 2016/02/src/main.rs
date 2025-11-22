use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("invalid direction"),
        }
    }
}

struct Button {
    value: u8,
    up: Option<u8>,
    down: Option<u8>,
    left: Option<u8>,
    right: Option<u8>,
}

impl Button {
    fn direction(&mut self, direction: &Direction) -> &mut Option<u8> {
        match direction {
            Direction::Up => &mut self.up,
            Direction::Down => &mut self.down,
            Direction::Left => &mut self.left,
            Direction::Right => &mut self.right,
        }
    }

    fn configure(&mut self, direction: &Direction, btn: u8) {
        *self.direction(direction) = Some(btn);
    }

    fn with_value(value: u8) -> Self {
        Self {
            value,
            up: None,
            down: None,
            left: None,
            right: None,
        }
    }

    fn move_if_valid(&self, direction: &Direction) -> usize {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
        .unwrap_or(self.value) as usize
    }
}

struct Relationship {
    from_button: u8,
    to_button: u8,
    relationship: Direction,
}

impl Relationship {
    fn new(from_button: u8, to_button: u8, relationship: Direction) -> Self {
        Self {
            from_button,
            to_button,
            relationship,
        }
    }
}

fn part_one() -> usize {
    // Using 0..10 instead of 0..9 lets us keep the indexes the same as the values, it's probably
    // not the most efficient way to do it, but it simplified the code slightly.
    let mut buttons = (0..10).map(Button::with_value).collect::<Vec<_>>();

    // We only need to cover relationships which are down or right, anything else
    // we can calculate by reversing it. It would probably be more concise to just use index
    // calculations to calculate the relationships as needed in a 2d array, but I wanted
    // something a bit more interesting.
    [
        Relationship::new(1, 2, Direction::Right),
        Relationship::new(1, 4, Direction::Down),
        Relationship::new(2, 3, Direction::Right),
        Relationship::new(2, 5, Direction::Down),
        Relationship::new(3, 6, Direction::Down),
        Relationship::new(4, 5, Direction::Right),
        Relationship::new(4, 7, Direction::Down),
        Relationship::new(5, 6, Direction::Right),
        Relationship::new(5, 8, Direction::Down),
        Relationship::new(6, 9, Direction::Down),
        Relationship::new(7, 8, Direction::Right),
        Relationship::new(8, 9, Direction::Right),
    ]
    .iter()
    .for_each(|relationship| {
        buttons[relationship.from_button as usize]
            .configure(&relationship.relationship, relationship.to_button);

        buttons[relationship.to_button as usize].configure(
            &relationship.relationship.opposite(),
            relationship.from_button,
        );
    });

    BufReader::new(File::open("input.txt").expect("input file error"))
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut btn = &buttons[5];

            line.chars().for_each(|c| {
                btn = &buttons[btn.move_if_valid(&Direction::from_char(c))];
            });

            btn.value
        })
        .map(|value| (value + ('0' as u8)) as char)
        .collect::<String>()
        .parse::<usize>()
        .expect("int parse error")
}

fn main() {
    println!("{}", part_one());
}
