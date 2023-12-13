use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum = 0;
    let mut map: Vec<Vec<bool>> = vec![];
    let mut row_reflection = vec![];
    let mut col_reflection = vec![];
    let mut offset = 0;
    for (mut y, inp_line) in input.enumerate() {
        if inp_line.as_ref() == "" {
            sum += col_reflection
                .first()
                .copied()
                .or_else(|| row_reflection.first().map(|v| *v * 100))
                .unwrap();
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
                let r = col_reflection[idx];
                if r > x {
                    break;
                }
                if x > r * 2 - 1 || line[(r * 2 - 1) - x] == v {
                    idx += 1;
                } else {
                    col_reflection.remove(idx);
                }
            }
            if y == 0 && line.last().is_some_and(|p| *p == v) {
                col_reflection.push(x);
            }
            line.push(v);
        }
        let mut idx = 0;
        while idx < row_reflection.len() {
            let r = row_reflection[idx];
            if r > y {
                break;
            }
            if y > r * 2 - 1 || map[(r * 2 - 1) - y] == line {
                idx += 1;
            } else {
                row_reflection.remove(idx);
            }
        }
        if map.last().is_some_and(|p| *p == line) {
            row_reflection.push(y);
        }
        map.push(line);
    }
    sum += col_reflection
        .first()
        .copied()
        .or_else(|| row_reflection.first().map(|v| *v * 100))
        .unwrap();
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
        405
    );
}
