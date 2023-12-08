use std::collections::HashMap;
use std::io;

fn read_dirs(line: &[u8]) -> Box<[bool]> {
    line.iter().map(|c| *c == b'L').collect()
}

fn nr(inp: &[u8]) -> u32 {
    inp.iter().fold(0, |r, c| r * 64 + u32::from(c - b'0'))
}

fn calc(mut input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let dirs = read_dirs(input.next().unwrap().as_ref().as_bytes());
    assert_eq!(input.next().unwrap().as_ref(), "");

    let mut nodes = HashMap::new();
    let mut tracks = Vec::new();
    for line in input {
        let line = line.as_ref().as_bytes();
        let node = nr(&line[0..3]);
        if node % 64 == u32::from(b'A' - b'0') {
            tracks.push(node);
        }
        nodes.insert(node, (nr(&line[7..10]), nr(&line[12..15])));
    }
    let mut cycle = 1;
    for mut cur in &*tracks {
        let mut len = 0;
        loop {
            cur = if dirs[len % dirs.len()] {
                &nodes[cur].0
            } else {
                &nodes[cur].1
            };
            len += 1;
            if cur % 64 == u32::from(b'Z' - b'0') {
                break;
            }
        }
        let mut new_cycle = cycle;
        while new_cycle % len > 0 {
            new_cycle += cycle;
        }
        cycle = new_cycle;
    }
    cycle
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"
            .lines()
        ),
        6
    );
}
