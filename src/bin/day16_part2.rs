use std::io;

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

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut fields = vec![];
    for (y, line) in input.enumerate() {
        let line = line.as_ref().as_bytes();
        fields.push(vec![]);
        for c in line {
            fields[y].push(*c);
        }
    }
    let bounds = (fields.len(), fields[0].len());
    (0..fields[0].len())
        .map(|x| ((0, x), 1))
        .chain((0..fields[0].len()).map(|x| ((fields.len() - 1, x), 4)))
        .chain((0..fields.len()).map(|y| ((y, 0), 8)))
        .chain((0..fields.len()).map(|y| ((y, fields[0].len() - 1), 8)))
        .map(|(pos, dir)| {
            let mut field_tags = vec![vec![0; bounds.1]; bounds.0];
            field_tags[pos.0][pos.1] = dir;
            let mut paths_to_take = vec![];
            match fields[pos.0][pos.1] {
                b'|' => {
                    if dir == 2 || dir == 8 {
                        paths_to_take.push((pos, 1));
                        paths_to_take.push((pos, 4));
                    } else {
                        paths_to_take.push((pos, opposite(dir)));
                    }
                }
                b'-' => {
                    if dir == 1 || dir == 4 {
                        paths_to_take.push((pos, 2));
                        paths_to_take.push((pos, 8));
                    } else {
                        paths_to_take.push((pos, opposite(dir)));
                    }
                }
                b'.' => paths_to_take.push((pos, opposite(dir))),
                b'/' => paths_to_take.push((
                    pos,
                    match dir {
                        1 => 8,
                        2 => 4,
                        4 => 2,
                        8 => 1,
                        _ => unreachable!(),
                    },
                )),
                b'\\' => paths_to_take.push((
                    pos,
                    match dir {
                        1 => 2,
                        2 => 1,
                        4 => 8,
                        8 => 4,
                        _ => unreachable!(),
                    },
                )),
                _ => unreachable!(),
            }
            while let Some((out_pos, out_dir)) = paths_to_take.pop() {
                let Some((pos, dir)) = maybe_do_step(out_pos, bounds, out_dir) else {
                    continue;
                };
                if field_tags[pos.0][pos.1] & dir > 0 {
                    continue;
                }
                field_tags[pos.0][pos.1] |= dir;
                match fields[pos.0][pos.1] {
                    b'|' => {
                        if dir == 2 || dir == 8 {
                            paths_to_take.push((pos, 1));
                            paths_to_take.push((pos, 4));
                        } else {
                            paths_to_take.push((pos, out_dir));
                        }
                    }
                    b'-' => {
                        if dir == 1 || dir == 4 {
                            paths_to_take.push((pos, 2));
                            paths_to_take.push((pos, 8));
                        } else {
                            paths_to_take.push((pos, out_dir));
                        }
                    }
                    b'.' => paths_to_take.push((pos, out_dir)),
                    b'/' => paths_to_take.push((
                        pos,
                        match dir {
                            1 => 8,
                            2 => 4,
                            4 => 2,
                            8 => 1,
                            _ => unreachable!(),
                        },
                    )),
                    b'\\' => paths_to_take.push((
                        pos,
                        match dir {
                            1 => 2,
                            2 => 1,
                            4 => 8,
                            8 => 4,
                            _ => unreachable!(),
                        },
                    )),
                    _ => unreachable!(),
                }
            }
            if false {
                for (fields_row, tags_row) in fields.iter().zip(field_tags.iter()) {
                    for (field, tag) in fields_row.iter().zip(tags_row.iter()) {
                        let e = 27 as char;

                        if *tag > 0 {
                            eprint!("{e}[0;31m");
                        }
                        eprint!("{}", *field as char);
                        if *tag > 0 {
                            eprint!("{e}[0m");
                        }
                    }
                    eprintln!();
                }
                eprintln!();
            }
            field_tags
                .into_iter()
                .map(|r| r.into_iter().filter(|x| *x > 0).count())
                .sum()
        })
        .max()
        .unwrap()
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test_1() {
    assert_eq!(
        calc(
            ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....
"
            .lines()
        ),
        51
    );
}
