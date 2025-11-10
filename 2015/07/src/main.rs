use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operation {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    NOT,
    NOOP,
}

impl Operation {
    fn from_name(name: &str) -> Operation {
        match name {
            "AND" => Operation::AND,
            "OR" => Operation::OR,
            "LSHIFT" => Operation::LSHIFT,
            "RSHIFT" => Operation::RSHIFT,
            _ => panic!("invalid operation name"),
        }
    }

    fn execute(&self, lhs: u16, rhs: Option<u16>) -> u16 {
        match self {
            Operation::AND => lhs & rhs.unwrap(),
            Operation::OR => lhs | rhs.unwrap(),
            Operation::LSHIFT => lhs << rhs.unwrap(),
            Operation::RSHIFT => lhs >> rhs.unwrap(),
            Operation::NOT => !lhs,
            Operation::NOOP => lhs,
        }
    }
}

#[derive(Debug)]
enum InputType {
    Constant(u16),
    Gate(String),
}

#[derive(Debug)]
struct Gate {
    lhs: InputType,
    rhs: Option<InputType>,

    out: String,

    op: Operation,
}

impl Gate {
    fn new(line: String) -> Self {
        let split = line.split(" ").collect::<Vec<&str>>();

        if split.len() == 3 {
            // Known value for the gate

            Gate {
                lhs: if Gate::is_gate(split[0]) {
                    InputType::Gate(split[0].to_string())
                } else {
                    InputType::Constant(split[0].parse().expect("i16 parse fail for NOT Gate"))
                },
                rhs: None,
                out: split[2].to_string(),
                op: Operation::NOOP,
            }
        } else if split.len() == 4 {
            // NOT gate - these have a slightly different syntax.

            Gate {
                lhs: if Gate::is_gate(split[1]) {
                    InputType::Gate(split[1].to_string())
                } else {
                    InputType::Constant(split[1].parse().expect("i16 parse fail for NOT Gate"))
                },
                rhs: None,
                op: Operation::NOT,
                out: split[3].to_string(),
            }
        } else {
            Gate {
                lhs: if Gate::is_gate(split[0]) {
                    InputType::Gate(split[0].to_string())
                } else {
                    InputType::Constant(
                        split[0]
                            .parse()
                            .expect("i16 parse fail for 2 arg (LHS) Gate"),
                    )
                },
                rhs: if Gate::is_gate(split[2]) {
                    Some(InputType::Gate(split[2].to_string()))
                } else {
                    Some(InputType::Constant(
                        split[2]
                            .parse()
                            .expect("i16 parse fail for 2 arg (RHS) Gate"),
                    ))
                },
                op: Operation::from_name(split[1]),
                out: split[4].to_string(),
            }
        }
    }

    fn is_gate(name: &str) -> bool {
        name.chars().all(|c| c.is_ascii_alphabetic())
    }

    fn solve(&self, gates: &Vec<Gate>, mem: &mut HashMap<String, u16>) -> u16 {
        self.op.execute(
            match &self.lhs {
                InputType::Constant(c) => *c,
                InputType::Gate(s) => {
                    if mem.contains_key(s) {
                        *mem.get(s).unwrap()
                    } else {
                        let res = gates
                            .iter()
                            .find(|&g| g.out == s.to_string())
                            .expect(&format!("missing gate: {}", s))
                            .solve(gates, mem);
                        mem.insert(s.to_string(), res);
                        res
                    }
                }
            },
            match &self.rhs {
                Some(gate) => Some(match gate {
                    InputType::Constant(c) => *c,
                    InputType::Gate(s) => {
                        if mem.contains_key(s) {
                            *mem.get(s).unwrap()
                        } else {
                            let res = gates
                                .iter()
                                .find(|&g| g.out == s.to_string())
                                .expect(&format!("missing gate: {}", s))
                                .solve(gates, mem);
                            mem.insert(s.to_string(), res);
                            res
                        }
                    }
                }),
                None => None,
            },
        )
    }
}

fn part_one() -> u16 {
    let gates = BufReader::new(File::open("input.txt").expect("input file"))
        .lines()
        .filter_map(|l| match l {
            Ok(n) => Some(n),
            Err(_) => None,
        })
        .map(|line| Gate::new(line))
        .collect::<Vec<_>>();

    let a = gates
        .iter()
        .find(|gate| gate.out == "a")
        .expect("gate a not found");

    a.solve(&gates, &mut HashMap::new())
}

fn main() {
    println!("{}", part_one());
}
