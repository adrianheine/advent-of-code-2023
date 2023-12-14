use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut roll_target = vec![];
    let mut rocks_per_row = vec![];
    for (y, inp_line) in input.enumerate() {
        rocks_per_row.push(0);
        for (x, c) in inp_line.as_ref().bytes().enumerate() {
            if y == 0 {
                roll_target.push(0);
            }
            match c {
                b'#' => roll_target[x] = y + 1,
                b'O' => {
                    rocks_per_row[roll_target[x]] += 1;
                    roll_target[x] += 1;
                }
                _ => {}
            }
        }
    }
    let rows = rocks_per_row.len();
    rocks_per_row
        .into_iter()
        .enumerate()
        .fold(0, |sum, (i, n)| sum + n * (rows - i))
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"
            .lines()
        ),
        136
    );
}
