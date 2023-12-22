use std::collections::HashSet;
use std::io;

fn eat(line: &mut &[u8], c: u8) {
    let mut idx = 0;
    while idx < line.len() && line[idx] == c {
        idx += 1;
    }
    *line = &line[idx..];
}

fn get_number(line: &mut &[u8]) -> usize {
    let mut n = 0;
    let mut idx = 0;
    while idx < line.len() && line[idx].is_ascii_digit() {
        n = n * 10 + (line[idx] - b'0') as usize;
        idx += 1;
    }
    *line = &line[idx..];
    n
}

#[derive(Debug)]
struct Brick {
    from: (usize, usize, usize),
    to: (usize, usize, usize),
}

fn read_values(mut line: &[u8]) -> Brick {
    Brick {
        from: (
            get_number(&mut line),
            {
                eat(&mut line, b',');
                get_number(&mut line)
            },
            {
                eat(&mut line, b',');
                get_number(&mut line)
            },
        ),
        to: (
            {
                eat(&mut line, b'~');
                eat(&mut line, b',');
                get_number(&mut line)
            },
            {
                eat(&mut line, b',');
                get_number(&mut line)
            },
            {
                eat(&mut line, b',');
                get_number(&mut line)
            },
        ),
    }
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut bricks = vec![];
    let mut bounds = (0, 0);
    for line in input {
        let line = line.as_ref().as_bytes();
        let brick = read_values(line);
        bounds.0 = bounds.0.max(brick.to.0 + 1);
        bounds.1 = bounds.1.max(brick.to.1 + 1);
        bricks.push(brick);
    }
    let mut supporting: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    let mut supported_by: Vec<HashSet<usize>> = vec![HashSet::new(); bricks.len()];
    let mut grid: Vec<Vec<Option<(usize, usize)>>> = Vec::with_capacity(bounds.0);
    for _ in 0..bounds.0 {
        let mut inner = Vec::with_capacity(bounds.1);
        for _ in 0..bounds.1 {
            inner.push(None);
        }
        grid.push(inner);
    }

    bricks.sort_by_key(|brick| brick.from.2);
    for (id, brick) in bricks.iter().enumerate() {
        let mut rest_at = 1;
        for x in brick.from.0..=brick.to.0 {
            for y in brick.from.1..=brick.to.1 {
                if let Some((other_id, height)) = &grid[x][y] {
                    if *height > rest_at {
                        rest_at = *height;
                        supported_by[id] = HashSet::from([*other_id]);
                    } else if *height == rest_at {
                        supported_by[id].insert(*other_id);
                    }
                }
            }
        }
        for x in brick.from.0..=brick.to.0 {
            for y in brick.from.1..=brick.to.1 {
                grid[x][y] = Some((id, rest_at + 1 + brick.to.2 - brick.from.2));
            }
        }
        for s in &supported_by[id] {
            supporting[*s].push(id);
        }
    }

    let mut sum = 0;
    for id in 0..supporting.len() {
        let mut dropped = HashSet::from([id]);
        let mut not_dropped: HashSet<_> = (0..supporting.len()).filter(|i| *i != id).collect();
        loop {
            let new_dropped: HashSet<_> = not_dropped
                .iter()
                .filter(|drop_candidate| {
                    !supported_by[**drop_candidate].is_empty()
                        && supported_by[**drop_candidate]
                            .iter()
                            .all(|support| dropped.contains(support))
                })
                .copied()
                .collect();
            if new_dropped.is_empty() {
                break;
            }
            for i in new_dropped {
                dropped.insert(i);
                not_dropped.remove(&i);
            }
        }
        sum += dropped.len() - 1;
    }
    sum
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"
            .lines()
        ),
        7
    );
}
