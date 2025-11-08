use std::fs::File;
use std::io::Read;

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn parse(src: &mut impl Iterator<Item = usize>) -> Option<Node> {
        let num_children = match src.next() {
            Some(num) => num,
            None => return None,
        };
        let num_metadata = match src.next() {
            Some(num) => num,
            None => return None,
        };

        Some(Node {
            children: (0..num_children)
                .map(|_| Node::parse(src).expect("node parse"))
                .collect::<Vec<_>>(),
            metadata: (0..num_metadata)
                .map(|_| src.next().expect("metadata"))
                .collect(),
        })
    }

    fn sum_metadata(&self) -> usize {
        self.children
            .iter()
            .map(|c| c.sum_metadata())
            .sum::<usize>()
            + self.metadata.iter().sum::<usize>()
    }
}

fn part_one() {
    let mut s = String::new();
    File::open("input.txt")
        .expect("input file")
        .read_to_string(&mut s)
        .expect("read error");

    let noded_tree = Node::parse(
        &mut s.split(" ")
            .map(|n| n.parse::<usize>().unwrap())
    );

    match noded_tree {
        Some(n) => {
            println!("{}", n.sum_metadata());
        }
        None => {
            println!("error");
        }
    }
}

fn main() {
    part_one();
}
