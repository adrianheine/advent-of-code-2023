use std::io::{self, Read};

const SPELLINGS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];
#[derive(Debug)]
struct Tree(Box<[(u8, Option<Tree>)]>);

fn build_tree(inp: &[u8], v: u8) -> (u8, Option<Tree>) {
    if inp.is_empty() {
        (v, None)
    } else {
        (inp[0], Some(Tree(Box::new([build_tree(&inp[1..], v)]))))
    }
}

fn build_tree_2(inp1: &[u8], v1: u8, inp2: &[u8], v2: u8) -> (u8, Option<Tree>) {
    (
        inp1[0],
        Some(Tree(if inp1[1] == inp2[1] {
            Box::new([build_tree_2(&inp1[1..], v1, &inp2[1..], v2)])
        } else {
            Box::new([build_tree(&inp1[1..], v1), build_tree(&inp2[1..], v2)])
        })),
    )
}

fn build_tree_multi(inps: &[&[u8]]) -> Tree {
    let mut inner = Vec::with_capacity(inps.len());
    let mut i = 0;
    while i < inps.len() {
        if i < inps.len() - 1 && inps[i][0] == inps[i + 1][0] {
            inner.push(build_tree_2(
                inps[i],
                (i + 1) as u8,
                inps[i + 1],
                (i + 2) as u8,
            ));
            i += 1;
        } else {
            inner.push(build_tree(inps[i], i as u8 + 1));
        }
        i += 1;
    }
    Tree(inner.into())
}

fn part_two(input: impl Iterator<Item = u8>) -> usize {
    let root: Tree = build_tree_multi(&SPELLINGS);

    let mut trees: [&Tree; 3] = [&root, &root, &root];
    let mut sum: usize = 0;
    let mut last: Option<u8> = None;
    for c in input {
        if c.is_ascii_digit() {
            let v = c - b'0';
            if last.is_none() {
                sum += 10 * v as usize;
            }
            last = Some(v);
            trees = [&root, &root, &root];
        } else if c == b'\n' || c == b'\r' {
            sum += last.unwrap() as usize;
            last = None;
            trees = [&root, &root, &root];
        } else {
            let mut did_root = false;
            'trees: for tree in &mut trees {
                if std::ptr::eq(*tree, &root) {
                    if did_root {
                        continue;
                    }
                    did_root = true;
                }
                for b in &*tree.0 {
                    if c == b.0 {
                        *tree = b.1.as_ref().unwrap(); // Descend
                        if (*tree.0)[0].1.is_none() {
                            let v = tree.0[0].0;
                            if last.is_none() {
                                sum += 10 * v as usize;
                            }
                            last = Some(v);
                            *tree = &root; // We're actually done here
                        }
                        continue 'trees; // No need to further look at this branch
                    }
                }
                *tree = &root; // Branch didn't match, so overwrite
            }
            assert!(did_root);
        }
    }
    sum
}

fn main() {
    println!(
        "{}",
        part_two(io::stdin().lock().bytes().map(Result::unwrap))
    );
}

#[test]
fn test_part_two() {
    assert_eq!(
        part_two(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"
            .bytes()
        ),
        281
    );
    assert_eq!(
        part_two("three4two4rnnslsvxmsbcpvnbpfseveneightwokcn\n".bytes()),
        32
    );
    assert_eq!(part_two("1one3gx3eight2\n".bytes()), 12);
    assert_eq!(part_two("threeone9rltsqbjl58zxxtktwoneh\n".bytes()), 31);
    assert_eq!(part_two("lfoneight4\n".bytes()), 14);
    assert_eq!(part_two("sevennine7eightpmlxqprzvjone\n".bytes()), 71);
}
