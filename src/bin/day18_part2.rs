use std::collections::VecDeque;
use std::io;

fn get_hex_number(line: &[u8]) -> isize {
    isize::from_str_radix(std::str::from_utf8(line).unwrap(), 16).unwrap()
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
        let mut idx = 0;
        while line[idx] != b' ' {
            idx += 1;
        }
        idx += 1;
        while line[idx] != b' ' {
            idx += 1;
        }
        idx += 3;
        let dist = get_hex_number(&line[idx..line.len() - 2]);
        let dir = match line[line.len() - 2] {
            b'3' => 1,
            b'0' => 2,
            b'1' => 4,
            b'2' => 8,
            _ => unreachable!(),
        };
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
    //let mut is_first_row = true;
    //let mut last_row = vec![];
    for mut row in points {
        //assert!(last_row.is_empty());
        row.sort_by_key(|(a, _)| *a);
        /*
        let mut draw_row = &row[0..];
        let mut ins = false;
        for x in smallest..=biggest {
            eprint!(
                "{}",
                if let Some(head) = draw_row.get(0) {
                    if x < head.0 {
                        if ins == true {
                          char::from_digit((x.abs() % 10) as u32, 10).unwrap()
                        } else {
                            '.'
                        }
                    } else if x >= head.0 && x <= head.0 + head.1 as isize {
                        '#'
                    } else {
                        draw_row = &draw_row[1..];
                        ins = !ins;
                        if ins == true {
                            if draw_row.get(0).is_some_and(|r| r.0 == head.0) {
                                '.'
                            } else {
                          char::from_digit((x.abs() % 10) as u32, 10).unwrap()
                            }
                        } else {
                            '.'
                        }
                    }
                } else {
                    '.'
                }
            );
        }
        eprint!("{row:?}");
        */
        let mut last_pos = isize::MAX;
        while let (Some((b, b_w)), Some((a, a_w))) = (row.pop(), row.pop()) {
            let end = if a == b {
                a + a_w as isize + b_w as isize + 1
            } else {
                b + b_w as isize + 1
            };
            let v = usize::try_from(end.min(last_pos) - a).unwrap();
            //eprint!(" {v}");
            sum += v;
            last_pos = a;
        }
        //eprintln!();
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
        952_408_144_115
    );
}
