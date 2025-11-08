use std::fs::File;
use std::io::Read;

fn react(n: String) -> String {
    let mut s = n;
    let mut has_changed = true;

    while has_changed {
        has_changed = false;

        let mut new_string = String::with_capacity(s.len());

        let mut chars = s.bytes();

        let mut c = chars.next();

        loop {
            match c {
                Some(a) => {
                    c = chars.next();
                    match c {
                        Some(b) => {
                            if a ^ b != 32 {
                                new_string.push(a as char);
                            } else {
                                has_changed = true;
                                c = chars.next();
                            }
                        },
                        None => {
                            new_string.push(a as char)
                        }
                    }
                },
                None => break,
            }
        }

        s = new_string;
    }

    s
}

fn part_one() -> usize {
    let mut s = String::new();
    File::open("input.txt")
        .expect("input file")
        .read_to_string(&mut s)
        .expect("read error");

    react(s).len()
}

fn main() {
    println!("{}", part_one());
}
