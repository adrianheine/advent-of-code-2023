use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut row_count = 0;
    let mut columns = vec![];
    for (y, inp_line) in input.enumerate() {
        if y == 0 {
            row_count = inp_line.as_ref().bytes().len();
        }
        for (x, c) in inp_line.as_ref().bytes().enumerate() {
            if y == 0 {
                columns.push(vec![(0, 0)]);
            }
            match c {
                b'#' => {
                    columns[x].push((y + 1, 0));
                }
                b'O' => {
                    columns[x].last_mut().unwrap().1 += 1;
                }
                _ => {}
            }
        }
    }
    let mut cycles = vec![];
    let mut cycle_length = usize::MAX;
    let mut cycle_start = usize::MAX;
    'cycles: for i in 0..1_000_000_000 * 4_u64 + 2 {
        let mut new_columns = vec![vec![(0, 0)]; row_count];
        for (x, column) in columns.iter().enumerate() {
            let mut y = 0;
            for &(start, rocks) in column {
                while y + 1 < start {
                    y += 1;
                }
                if start > 0 {
                    new_columns[row_count - (start - 1) - 1].push((x + 1, 0));
                    y += 1;
                }
                while y < start + rocks {
                    new_columns[row_count - y - 1].last_mut().unwrap().1 += 1;
                    y += 1;
                }
            }
        }
        row_count = columns.len();
        columns = new_columns;
        if i % 4 == 2 {
            for (other_id, other) in cycles.iter().enumerate().rev() {
                if *other == columns {
                    cycle_length = cycles.len() - other_id;
                    cycle_start = other_id;
                    break 'cycles;
                }
            }
            cycles.push(columns.clone());
        }
    }
    if cycle_start < usize::MAX {
        columns = cycles[cycle_start + (1_000_000_000 - cycle_start - 1) % cycle_length].clone();
    }

    let mut sum = 0;
    let row_count = columns.len();
    for (x, column) in columns.iter().enumerate() {
        sum += column
            .iter()
            .map(|(_, c)| c * (row_count - x))
            .sum::<usize>();
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
        64
    );
}
