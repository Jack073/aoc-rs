use itertools::Itertools;

fn part_one() -> usize {
    (359282..820401)
        .map(|n| {
            [
                n / 100000,
                (n / 10000) % 10,
                (n / 1000) % 10,
                (n / 100) % 10,
                (n / 10) % 10,
                n % 10,
            ]
        })
        .filter(|n| n.is_sorted())
        .filter(|n| n
            .iter()
            .chunk_by(|&k| k)
            .into_iter()
            .any(|b| b.1.count() >= 2))
        .count()
}

fn part_two() -> usize {
    (359282..820401)
        .map(|n| {
            [
                n / 100000,
                (n / 10000) % 10,
                (n / 1000) % 10,
                (n / 100) % 10,
                (n / 10) % 10,
                n % 10,
            ]
        })
        .filter(|n| n.is_sorted())
        .filter(|n| n
            .iter()
            .chunk_by(|&k| k)
            .into_iter()
            .any(|b| b.1.count() == 2))
        .count()
}

fn main() {
    println!("{}", part_two());
}
