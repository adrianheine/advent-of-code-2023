use std::io;

fn finish<'a>(
    col: impl IntoIterator<Item = &'a (usize, Option<(usize, usize)>)>,
    row: impl IntoIterator<Item = &'a (usize, Option<(usize, usize)>)>,
) -> usize {
    col.into_iter()
        .find(|(_, smudge)| smudge.is_some())
        .map(|(v, _)| *v)
        .xor(
            row.into_iter()
                .find(|(_, smudge)| smudge.is_some())
                .map(|(v, _)| *v * 100),
        )
        .unwrap()
}

fn diff_lines(a: &[bool], b: &[bool]) -> Option<usize> {
    let mut diffs = vec![];
    for (idx, (a, b)) in a.iter().zip(b.iter()).enumerate() {
        if a != b {
            diffs.push(idx);
        }
    }
    if diffs.len() == 1 {
        Some(diffs[0])
    } else {
        None
    }
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum = 0;
    let mut map: Vec<Vec<bool>> = vec![];
    let mut row_reflection = vec![];
    let mut col_reflection = vec![];
    let mut offset = 0;
    for (mut y, inp_line) in input.enumerate() {
        if inp_line.as_ref() == "" {
            sum += finish(&col_reflection, &row_reflection);
            offset = y + 1;
            map = vec![];
            row_reflection = vec![];
            col_reflection = vec![];
            continue;
        }
        y -= offset;
        let mut line: Vec<bool> = vec![];
        for (x, c) in inp_line.as_ref().bytes().enumerate() {
            let v = b'#' == c;
            let mut idx = 0;
            while idx < col_reflection.len() {
                let (r, smudge) = col_reflection[idx];
                if r > x {
                    break;
                }
                if x > r * 2 - 1 || line[(r * 2 - 1) - x] == v {
                    idx += 1;
                } else if smudge.is_none() {
                    col_reflection[idx].1 = Some((y, x));
                    idx += 1;
                } else {
                    col_reflection.remove(idx);
                }
            }
            if y == 0 && x > 0 {
                col_reflection.push((
                    x,
                    line.last()
                        .map(|p| if *p == v { None } else { Some((y, x - 1)) })
                        .unwrap(),
                ));
            }
            line.push(v);
        }
        let mut idx = 0;
        while idx < row_reflection.len() {
            let (r, smudge) = row_reflection[idx];
            if r > y {
                break;
            }
            if y > r * 2 - 1 || map[(r * 2 - 1) - y] == line {
                idx += 1;
            } else if smudge.is_some() {
                row_reflection.remove(idx);
            } else if let Some(diff) = diff_lines(&line, &map[(r * 2 - 1) - y]) {
                row_reflection[idx].1 = Some((y, diff));
                idx += 1;
            } else {
                row_reflection.remove(idx);
            }
        }
        if y > 0 {
            if map.last().map(|p| *p == line).unwrap() {
                row_reflection.push((y, None));
            } else if let Some(diff) = diff_lines(&line, map.last().unwrap()) {
                row_reflection.push((y, Some((y, diff))));
            }
        }
        map.push(line);
    }
    sum += finish(&col_reflection, &row_reflection);
    sum
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"
            .lines()
        ),
        400
    );
}
