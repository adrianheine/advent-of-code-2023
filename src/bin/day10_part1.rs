use std::io;

fn read_field(field: &u8) -> usize {
    match field {
        b'|' => 1 | 4,
        b'-' => 2 | 8,
        b'L' => 1 | 2,
        b'J' => 1 | 8,
        b'7' => 4 | 8,
        b'F' => 2 | 4,
        b'.' => 0,
        b'S' => 16,
        _ => unreachable!(),
    }
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> isize {
    let mut start = None;
    let mut y = 0;
    let mut fields = vec![];
    for line in input {
        let line = line.as_ref().as_bytes();
        fields.push(vec![]);
        let mut x = 0;
        for c in line {
            let field = read_field(c);
            if field & 16 == 16 {
                start = Some((y, x));
            }
            fields[y].push(field);
            x += 1;
        }
        y += 1;
    }
    let mut cur = start.unwrap();
    let mut dir = if cur.1 > 0 && (fields[cur.0][cur.1 - 1] & 2 == 2) {
        8
    } else if cur.1 < fields[cur.0].len() && (fields[cur.0][cur.1 + 1] & 8 == 8) {
        2
    } else if cur.0 > 0 && (fields[cur.0 - 1][cur.1] & 4 == 4) {
        1
    } else if cur.0 < fields.len() && (fields[cur.0 + 1][cur.1] & 1 == 1) {
        4
    } else {
        unreachable!()
    };
    let mut length = 0;
    loop {
        length += 1;
        (cur, dir) = match dir {
            8 => ((cur.0, cur.1 - 1), 2),
            2 => ((cur.0, cur.1 + 1), 8),
            1 => ((cur.0 - 1, cur.1), 4),
            4 => ((cur.0 + 1, cur.1), 1),
            _ => unreachable!(),
        };
        dir = fields[cur.0][cur.1] & !dir;
        if dir == 16 {
            break;
        }
    }
    length / 2
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test_simple_square() {
    assert_eq!(
        calc(
            ".....
.S-7.
.|.|.
.L-J.
.....
"
            .lines()
        ),
        4
    );
}

#[test]
fn test_simple_square_with_extras() {
    assert_eq!(
        calc(
            "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"
            .lines()
        ),
        4
    );
}

#[test]
fn test_complex() {
    assert_eq!(
        calc(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"
            .lines()
        ),
        8
    );
}

#[test]
fn test_complex_with_extras() {
    assert_eq!(
        calc(
            "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"
            .lines()
        ),
        8
    );
}
