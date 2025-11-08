use std::fs::File;
use std::io::Read;

fn compute(opcodes: &mut Vec<usize>, first: usize, second: usize) -> usize {
    let mut ptr = 0;

    opcodes[1] = first;
    opcodes[2] = second;

    loop {
        let opcode = opcodes[ptr];
        if opcode == 99 {
            break;
        }

        let arg1 = opcodes[ptr + 1];
        let arg2 = opcodes[ptr + 2];
        let output = opcodes[ptr + 3];

        ptr += 4;

        opcodes[output] = match opcode {
            1 => opcodes[arg1] + opcodes[arg2],
            2 => opcodes[arg1] * opcodes[arg2],
            _ => panic!("invalid opcode")
        };
    }

    opcodes[0]
}

fn part_one() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .expect("input file")
        .read_to_string(&mut s)
        .expect("read error");

    let mut opcodes = s
        .split(",")
        .map(|n| n.parse::<usize>().expect("parse err"))
        .collect::<Vec<_>>();

    compute(&mut opcodes, 12, 2)
}

fn part_two() -> usize {
    let mut s = String::new();

    File::open("input.txt")
        .expect("input file")
        .read_to_string(&mut s)
        .expect("read error");

    let opcodes = s
        .split(",")
        .map(|n| n.parse::<usize>().expect("parse err"))
        .collect::<Vec<_>>();

    for noun in 0..=99 {
        for verb in 0..=99 {
            if compute(&mut opcodes.clone(), noun, verb) == 19690720 {
                return verb + (100 * noun);
            }
        }
    }
    panic!("no solution found")
}

fn main() {
    println!("{}", part_two());
}
