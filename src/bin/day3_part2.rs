use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum: usize = 0;
    let mut prev_maybe_gears: Box<[(usize, Vec<usize>)]> = Box::default();
    let mut prev_numbers: Box<[(usize, usize, usize)]> = Box::default();
    let mut prev_numbers_offset = 0;
    for line in input {
        let mut maybe_gears = vec![];
        let mut numbers: Vec<(usize, usize, usize)> = vec![];
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
                if line[idx] != b'*' {
                    idx += 1;
                    continue;
                }
                // Actually not a number, but maybe a gear
                let mut adj_numbers = Vec::with_capacity(2);
                if !numbers.is_empty() && numbers[numbers.len() - 1].1 == idx - 1 {
                    adj_numbers.push(numbers[numbers.len() - 1].2);
                }
                let mut pos = prev_numbers_offset;
                while pos < prev_numbers.len() && prev_numbers[pos].0 <= idx + 1 {
                    if prev_numbers[pos].0 <= idx + 1 && idx <= prev_numbers[pos].1 + 1 {
                        adj_numbers.push(prev_numbers[pos].2);
                    }
                    pos += 1;
                }
                prev_numbers_offset = pos;
                if adj_numbers.len() <= 2 {
                    maybe_gears.push((idx, adj_numbers));
                }
                idx += 1;
            } else {
                numbers.push((number_start, idx - 1, n));
                if let Some(maybe_gear) = maybe_gears.last_mut() {
                    if maybe_gear.0 == number_start - 1 {
                        maybe_gear.1.push(n);
                    }
                }
                for maybe_gear in &mut prev_maybe_gears.iter_mut() {
                    if number_start <= maybe_gear.0 + 1 && maybe_gear.0 <= idx {
                        maybe_gear.1.push(n);
                    }
                }
            }
        }
        for (_, numbers) in &*prev_maybe_gears {
            if numbers.len() == 2 {
                sum += numbers[0] * numbers[1];
            }
        }
        prev_maybe_gears = maybe_gears.into();
        prev_numbers = numbers.into();
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
        467_835
    );
    assert_eq!(calc("x1".lines()), 0);
}
