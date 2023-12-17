use std::io;

fn read_field(field: u8) -> usize {
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

fn opposite(dir: usize) -> usize {
    match dir {
        1 => 4,
        2 => 8,
        4 => 1,
        8 => 2,
        _ => unreachable!(),
    }
}

fn vec(dir: usize) -> (isize, isize) {
    match dir {
        1 => (-1, 0),
        2 => (0, 1),
        4 => (1, 0),
        8 => (0, -1),
        _ => unreachable!(),
    }
}

fn maybe_do_step(
    pos: (usize, usize),
    bounds: (usize, usize),
    dir: usize,
) -> Option<((usize, usize), usize)> {
    let v = vec(dir);
    if (pos.0 > 0 || v.0 >= 0)
        && (pos.1 > 0 || v.1 >= 0)
        && (pos.0 < bounds.0 - 1 || v.0 <= 0)
        && (pos.1 < bounds.1 - 1 || v.1 <= 0)
    {
        Some((
            (
                (pos.0 as isize + v.0) as usize,
                (pos.1 as isize + v.1) as usize,
            ),
            opposite(dir),
        ))
    } else {
        None
    }
}

fn write_field(field: usize) -> u8 {
    match field {
        5 => b'|',
        10 => b'-',
        3 => b'L',
        9 => b'J',
        12 => b'7',
        6 => b'F',
        0 => b'.',
        _ => unreachable!(),
    }
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut start = None;
    let mut fields = vec![];
    for (y, line) in input.enumerate() {
        let line = line.as_ref().as_bytes();
        fields.push(vec![]);
        for (x, c) in line.iter().enumerate() {
            let field = read_field(*c);
            if field & 16 == 16 {
                start = Some((y, x));
            }
            fields[y].push(field);
        }
    }
    let bounds = (fields.len(), fields[0].len());
    let mut cur = start.unwrap();
    let mut dir = [1, 2, 4, 8]
        .into_iter()
        .find(|dir| {
            maybe_do_step(cur, bounds, *dir)
                .is_some_and(|(next, opposite_dir)| fields[next.0][next.1] & opposite_dir > 0)
        })
        .unwrap();
    fields[cur.0][cur.1] |= dir;
    // 32 => part of the loop
    // 64 => out of the loop, seen
    // 1 | 2 | 4 | 8 => part of the loop, seen from this direction
    let mut field_tags = vec![vec![0; bounds.1]; bounds.0];
    let mut loop_length = 0;
    loop {
        loop_length += 1;
        field_tags[cur.0][cur.1] = 32;
        let prev_dir;
        (cur, prev_dir) = maybe_do_step(cur, bounds, dir).unwrap();
        dir = fields[cur.0][cur.1] & !prev_dir;
        if dir & 16 > 0 {
            fields[cur.0][cur.1] |= prev_dir;
            fields[cur.0][cur.1] &= !16;
            break;
        }
    }
    let mut paths_to_take = vec![
        ((0, 0), 2, if fields[0][0] == 6 { 4 } else { 0 }),
        ((0, 0), 4, if fields[0][0] == 6 { 2 } else { 0 }),
    ];
    let mut count = 0;
    while let Some((out_cur, out_dir, mut closed_at)) = paths_to_take.pop() {
        let Some((cur, dir)) = maybe_do_step(out_cur, bounds, out_dir) else {
            continue;
        };
        let mut new_closed_at: Box<dyn Fn(usize) -> usize> = Box::new(|_| 0);
        match field_tags[cur.0][cur.1] {
            0 => {
                field_tags[cur.0][cur.1] = 64;
                count += 1;
                closed_at = 0;
            }
            64 => continue,
            field if field & dir == dir => continue,
            _ => {
                field_tags[cur.0][cur.1] |= dir;
                let field = fields[cur.0][cur.1];
                let op = (1 | 2 | 4 | 8) ^ field;
                if field == 5 || field == 10 {
                    closed_at = (closed_at | out_dir) & !field;
                    new_closed_at = Box::new(move |d| if op & d > 0 { 0 } else { closed_at });
                } else if (closed_at & !out_dir) & op > 0 {
                    closed_at |= out_dir | op;
                    new_closed_at = Box::new(|_| out_dir);
                } else {
                    closed_at = 0;
                    new_closed_at = Box::new(move |d| if op & d > 0 { 0 } else { field });
                }
            }
        }
        for i in [1, 2, 4, 8] {
            if dir != i && (closed_at & i == 0) {
                paths_to_take.push((cur, i, new_closed_at(i)));
            }
        }
    }
    if false {
        for (fields_row, tags_row) in fields.iter().zip(field_tags.iter()) {
            for (field, tag) in fields_row.iter().zip(tags_row.iter()) {
                let e = 27 as char;

                if *tag & 32 > 0 {
                    eprint!("{e}[0;31m");
                }
                eprint!(
                    "{}",
                    if *tag & 32 == 0 && *tag != 64 {
                        'I'
                    } else {
                        write_field(*field) as char
                    }
                );
                if *tag & 32 > 0 {
                    eprint!("{e}[0m");
                }
            }
            eprintln!();
        }
    }
    field_tags.len() * field_tags[0].len() - count - loop_length
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test_1() {
    assert_eq!(
        calc(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
                .lines()
        ),
        4
    );
}

#[test]
fn test_2() {
    assert_eq!(
        calc(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"
            .lines()
        ),
        8
    );
}

#[test]
fn test_3() {
    assert_eq!(
        calc(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"
            .lines()
        ),
        10
    );
}

#[test]
fn test_4() {
    assert_eq!(
        calc(
            "...................
...................
F--------------7...
|F------7F----7|...
||F--7.FJL7F-7||.F7
||L--J.|F7LJFJ|L7||
||F--7FJ||F7L7L-JLJ
||L--JL-JLJL-JJF7LF
||F7.F---------JL7J
|LJL7|F----7F-7F-JF
L--7LJL---7||FLJ-|L
...S------JLJ....L-
"
            .lines()
        ),
        0
    );
}
