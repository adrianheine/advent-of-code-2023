use std::collections::VecDeque;
use std::io;

fn get_number(line: &[u8]) -> isize {
    let mut n = 0;
    let mut idx = 0;
    while idx < line.len() && line[idx].is_ascii_digit() {
        n = n * 10 + (line[idx] - b'0') as isize;
        idx += 1;
    }
    n
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut zero_at = 0;
    let mut points = VecDeque::new();
    points.push_back(vec![(0, 0)]);
    let mut pos: (isize, isize) = (0, 0);
    let mut entered_from = 0;
    let mut smallest = 0;
    let mut biggest = 0;
    for line in input {
        let line = line.as_ref().as_bytes();
        let dir = match line[0] {
            b'U' => 1,
            b'R' => 2,
            b'D' => 4,
            b'L' => 8,
            _ => unreachable!(),
        };
        let dist = get_number(&line[2..]);
        match dir {
            1 => {
                if entered_from != 1 {
                    points[(pos.0 + zero_at) as usize].push((pos.1, 0));
                }
                for i in 1..=dist {
                    pos.0 -= 1;
                    if -pos.0 > zero_at {
                        points.push_front(vec![]);
                        zero_at += 1;
                    }
                    if i < dist {
                        points[(pos.0 + zero_at) as usize].push((pos.1, 0));
                    }
                }
                entered_from = 1;
            }
            2 => {
                points[(pos.0 + zero_at) as usize].push((pos.1, dist as usize));
                pos.1 += dist;
                biggest = biggest.max(pos.1);
            }
            4 => {
                if entered_from != 4 {
                    points[(pos.0 + zero_at) as usize].push((pos.1, 0));
                }
                for i in 1..=dist {
                    pos.0 += 1;
                    if points.len() < (pos.0 + zero_at + 1) as usize {
                        points.push_back(vec![]);
                    }
                    if i < dist {
                        points[(pos.0 + zero_at) as usize].push((pos.1, 0));
                    }
                }
                entered_from = 4;
            }
            8 => {
                pos.1 -= dist;
                points[(pos.0 + zero_at) as usize].push((pos.1, dist as usize));
                smallest = smallest.min(pos.1);
            }
            _ => unreachable!(),
        }
    }
    assert_eq!(pos, (0, 0));

    let mut sum = 0;
    for mut row in points {
        row.sort_by_key(|(a, _)| *a);
        let mut last_pos = isize::MAX;
        while let (Some((b, b_w)), Some((a, a_w))) = (row.pop(), row.pop()) {
            let end = if a == b {
                a + a_w as isize + b_w as isize + 1
            } else {
                b + b_w as isize + 1
            };
            let v = usize::try_from(end.min(last_pos) - a).unwrap();
            sum += v;
            last_pos = a;
        }
    }
    sum
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test_1() {
    assert_eq!(
        calc(
            "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"
            .lines()
        ),
        62
    );
}

#[test]
fn test_2() {
    assert_eq!(
        calc(
            "R 6 (#0e4c90)
U 2 (#5b83a3)
R 7 (#1a3a90)
U 2 (#64e9b1)
R 8 (#27d840)
D 7 (#069321)
R 3 (#08be10)
D 8 (#0cb9c3)
R 7 (#372230)
U 5 (#0cb9c1)
L 4 (#3474a0)
U 4 (#069323)
R 4 (#1c9780)
U 6 (#64e9b3)
R 5 (#013fd0)
U 3 (#4370b3)
R 29 ()
D 1
L 5 (#2b2ed2)
D 9 (#340df3)
L 11 (#30dc52)
D 6 (#32cbf3)
L 10 (#451942)
D 8 (#1cf853)
L 10 (#4f1e12)
D 3 (#32afd3)
L 4 (#4f1e10)
D 4 (#43d723)
R 12 (#209752)
D 6 (#4b2143)
L 12 (#65b090)
D 4 (#29ebd3)
L 4 (#02cb22)
U 9 (#380263)
L 5 (#643ff2)
U 6 (#4844d3)
L 7 (#376762)
U 5 (#11c443)
L 2 (#487de2)
U 7 (#6e1383)
L 7 (#184542)
U 7 (#12f1d3)
"
            .lines()
        ),
        1223
    );
}
