use itertools::Itertools;

fn look_and_say(inp: Vec<usize>, iterations: usize) -> Vec<usize> {
    if iterations == 0 {
        inp
    } else {
        look_and_say(
            inp.into_iter()
                .chunk_by(|&c| c)
                .into_iter()
                .map(|c| [c.1.count(), c.0])
                .flatten()
                .collect(),
            iterations - 1,
        )
    }
}

fn part_one() {
    println!(
        "{}",
        look_and_say(
            "1113222113"
                .chars()
                .map(|n| (n as u16 as usize) - '0' as u16 as usize)
                .collect::<Vec<_>>(),
            40
        )
        .len()
    );
}

fn part_two() {
    println!(
        "{}",
        look_and_say(
            "1113222113"
                .chars()
                .map(|n| (n as u16 as usize) - '0' as u16 as usize)
                .collect::<Vec<_>>(),
            50
        )
        .len()
    );
}

fn main() {
    part_two();
}
