use std::collections::HashMap;
use std::io;

fn read_dirs(line: &[u8]) -> Box<[bool]> {
    line.iter().map(|c| *c == b'L').collect()
}

fn calc(mut input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let dirs = read_dirs(input.next().unwrap().as_ref().as_bytes());
    assert_eq!(input.next().unwrap().as_ref(), "");

    let mut nodes = HashMap::new();
    for line in input {
        let line = line.as_ref().as_bytes();
        nodes.insert(
            line[0..3].to_vec(),
            (line[7..10].to_vec(), line[12..15].to_vec()),
        );
    }
    let mut step = 0;
    let mut cur: &[u8] = b"AAA";
    while cur != b"ZZZ" {
        cur = if dirs[step % dirs.len()] {
            &nodes[cur].0
        } else {
            &nodes[cur].1
        };
        step += 1;
    }
    step
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"
            .lines()
        ),
        2
    );
}
