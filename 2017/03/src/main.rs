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

fn main() {
    println!("{}", part_one());
}
