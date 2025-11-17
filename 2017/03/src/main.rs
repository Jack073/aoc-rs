fn part_one() -> usize {
    let input: usize = 368078;

    let sqrt = input.isqrt();
    let side_length = if sqrt % 2 == 0 { sqrt + 1 } else { sqrt };

    let right_bottom_corner = side_length.pow(2);
    let left_bottom_corner = right_bottom_corner - side_length + 1;
    let left_top_corner = left_bottom_corner - side_length + 1;
    let right_top_corner = right_bottom_corner - side_length + 1;

    let spiral_entry_number = right_top_corner - side_length + 2;

    let midpoint = side_length / 2;

    let (low, _) = [
        (left_bottom_corner, right_bottom_corner),
        (left_top_corner, left_bottom_corner),
        (right_top_corner, left_top_corner),
        (spiral_entry_number, right_top_corner),
    ]
    .into_iter()
    .find(|(low, high)| low <= &input && &input <= high)
    .expect("input out of outer layer bounds");

    (side_length / 2) + midpoint.abs_diff(input - low)
}

struct Spiral {
    matrix: Vec<Vec<Option<usize>>>,
    row: isize,
    col: isize,
    upper_bound: isize,
    side_len: usize,
    origin_row: isize,
    origin_col: isize,
}

impl Spiral {
    fn new(dimension: usize) -> Spiral {
        let origin = dimension as isize / 2;
        Spiral {
            matrix: vec![vec![None; dimension]; dimension],
            row: origin,
            col: origin,
            origin_row: origin,
            origin_col: origin,
            upper_bound: dimension as isize,
            side_len: 1,
        }
    }

    fn init_origin(&mut self) {
        self.matrix[self.origin_row as usize][self.origin_col as usize] = Some(1);
    }

    fn surrounding(&self) -> impl Iterator<Item = usize> {
        [
            (0isize, 1isize),
            (1, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ]
        .into_iter()
        .filter_map(|delta| {
            let new_row = self.row + delta.0;
            let new_col = self.col + delta.1;

            if 0 <= new_row
                && new_row <= self.upper_bound
                && 0 <= new_col
                && new_col <= self.upper_bound
            {
                self.matrix[new_row as usize][new_col as usize]
            } else {
                None
            }
        })
    }

    fn next_coord(&mut self) {
        let delta_x = self.row - self.origin_row;
        let delta_y = self.col - self.origin_col;

        let relative_side_len = self.side_len / 2;

        let (new_delta_x, new_delta_y) = {
            if relative_side_len == 0 {
                self.side_len += 2;
                (1, 0)
            } else {
                if delta_x.unsigned_abs() == relative_side_len {
                    if delta_y.unsigned_abs() == relative_side_len {
                        // We're at one of the corners, we need to change direction.

                        match (delta_x.signum(), delta_y.signum()) {
                            // Bottom right corner - ascend
                            (1, 1) => (0, -1),

                            // Top right corner - go left
                            (1, -1) => (-1, 0),

                            // Bottom left corner - go right
                            (-1, 1) => {
                                self.side_len += 2;
                                (1, 0)
                            }

                            // Top left corner - go down
                            (-1, -1) => (0, 1),
                            _ => panic!("signum check"),
                        }
                    } else {
                        // We know we're moving vertically, we can use the sign of the delta x
                        // to determine which vertical direction.
                        (0, -delta_x.signum())
                    }
                } else {
                    (delta_y.signum(), 0)
                }
            }
        };

        self.row += new_delta_x;
        self.col += new_delta_y;
    }

    fn update_value(&mut self) -> usize {
        let new_value = self.surrounding().sum::<usize>();
        self.matrix[self.row as usize][self.col as usize] = Some(new_value);

        new_value
    }
}

fn part_two() {
    // While I would prefer for part two to also have a mathematical solution,
    // part two was atleast also interesting to implement.

    let input = 368078;

    let mut spiral = Spiral::new(1000);
    spiral.init_origin();
    loop {
        spiral.next_coord();
        let val = spiral.update_value();
        if val > input {
            println!("{}", val);
            return;
        }
    }
}

fn main() {
    part_two();
}
