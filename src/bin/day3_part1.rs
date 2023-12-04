use std::cmp::Ordering;
use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum: usize = 0;
    let mut prev_symbols: Box<[usize]> = Box::default();
    let mut prev_numbers: Box<[(usize, usize, usize)]> = Box::default();
    let mut prev_numbers_offset = 0;
    for line in input {
        let mut symbols = vec![];
        let mut open_numbers: Vec<(usize, usize, usize)> = vec![];
        let line = line.as_ref().as_bytes();
        let mut idx = 0;
        while idx < line.len() {
            while idx < line.len() && line[idx] == b'.' {
                idx += 1;
            }
            if idx == line.len() {
                continue;
            }
            let mut n: usize = 0;
            let number_start = idx;
            while idx < line.len() && line[idx].is_ascii_digit() {
                n = n * 10 + (line[idx] - b'0') as usize;
                idx += 1;
            }
            if idx == number_start {
                // Actually not a number, but a symbol
                symbols.push(idx);
                if !open_numbers.is_empty() && open_numbers[open_numbers.len() - 1].1 == idx - 1 {
                    sum += open_numbers.pop().unwrap().2;
                }
                let mut pos = prev_numbers_offset;
                while pos < prev_numbers.len() && prev_numbers[pos].0 <= idx + 1 {
                    if prev_numbers[pos].0 <= idx + 1 && idx <= prev_numbers[pos].1 + 1 {
                        sum += prev_numbers[pos].2;
                    }
                    pos += 1;
                }
                prev_numbers_offset = pos;
                idx += 1;
            } else if (!symbols.is_empty() && symbols[symbols.len() - 1] == number_start - 1)
                || prev_symbols
                    .binary_search_by(|symbol| {
                        match (
                            symbol.cmp(&(number_start.saturating_sub(1))),
                            symbol.cmp(&idx),
                        ) {
                            (Ordering::Equal, _)
                            | (_, Ordering::Equal)
                            | (Ordering::Greater, Ordering::Less) => Ordering::Equal,
                            (Ordering::Less, _) => Ordering::Less,
                            (_, Ordering::Greater) => Ordering::Greater,
                        }
                    })
                    .is_ok()
            {
                sum += n;
            } else {
                open_numbers.push((number_start, idx - 1, n));
            }
        }
        prev_symbols = symbols.into();
        prev_numbers = open_numbers.into();
        prev_numbers_offset = 0;
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
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
            .lines()
        ),
        4361
    );
    assert_eq!(calc("x1".lines()), 1);
}
