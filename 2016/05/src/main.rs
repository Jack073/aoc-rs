use md5::compute;

fn part_one() -> String {
    let input = include_str!("../input.txt");

    (0..(usize::MAX))
        .map(|n| compute(format!("{}{}", input, n)))
        .filter_map(|digest| {
            let hex = format!("{:x}", digest);
            let check = &hex[0..5];
            let output = hex.chars().nth(5).unwrap();
            if check.chars().all(|c| c == '0') {
                Some(output)
            } else {
                None
            }
        })
        .take(8)
        .collect::<String>()
}

fn main() {
    println!("{}", part_one());
}
