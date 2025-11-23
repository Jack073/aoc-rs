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

fn part_two() -> String {
    let input = include_str!("../input.txt");

    let mut seen = [false; 8];

    let mut list = (0..(usize::MAX))
        .map(|n| compute(format!("{}{}", input, n)))
        .filter_map(|digest| {
            let hex = format!("{:x}", digest);
            let check = &hex[0..5];
            if check.chars().all(|c| c == '0') {
                let location_char = match hex.chars().nth(5) {
                    n @ Some('0'..='7') => n.unwrap(),
                    _ => return None,
                };

                let location = (location_char as usize) - ('0' as usize);
                if seen[location] {
                    return None;
                }

                seen[location] = true;
                Some((location, hex.chars().nth(6).unwrap()))
            } else {
                None
            }
        })
        .take(8)
        .collect::<Vec<_>>();

    list.sort_by_key(|n| n.0);

    list.iter().map(|n| n.1).collect::<String>()
}

fn main() {
    println!("{}", part_two());
}
