use std::fs::File;
use std::io::Read;

fn decompress(src: String) -> String {
    let mut output = String::new();

    let mut chars = src.chars();

    loop {
        match chars.next() {
            Some('(') => {
                let mut num_chars = 0usize;
                let mut next_char = chars.next().unwrap();
                while next_char != 'x' {
                    num_chars *= 10;
                    num_chars += ((next_char as u8) - ('0' as u8)) as usize;
                    next_char = chars.next().unwrap();
                }

                let mut num_repeats = 0usize;

                next_char = chars.next().unwrap();

                while next_char != ')' {
                    num_repeats *= 10;
                    num_repeats += ((next_char as u8) - ('0' as u8)) as usize;
                    next_char = chars.next().unwrap();
                }

                let repeated_section = chars.by_ref().take(num_chars).collect::<String>();
                output.push_str(&repeated_section.repeat(num_repeats));
            }
            Some(n) => output.push(n),
            None => break,
        }
    }

    output
}

fn part_one() -> usize {
    // We have two main options here:
    // - Build the full string output. This is probably the more computationally expensive option
    //      as it'll require actually building and storing the output, including allocating memory
    //      for it. Trying to think ahead though, I have a slight suspicion that part two might
    //      involve repeatedly decompressing the output, so this could save us some work.
    // - Since we only need the length, we can just calculate it without building the output.
    //
    // Since it doesn't look like there's anything too crazy in the input, I am going to opt for
    // the first option.

    let mut s = String::new();
    File::open("input.txt")
        .expect("input file read error")
        .read_to_string(&mut s)
        .unwrap();

    decompress(s).len()
}

fn main() {
    println!("{}", part_one());
}
