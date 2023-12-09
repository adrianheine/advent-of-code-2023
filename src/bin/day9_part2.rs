use std::io;

fn eat(line: &mut &[u8], c: u8) {
    let mut idx = 0;
    while idx < line.len() && line[idx] == c {
        idx += 1;
    }
    *line = &line[idx..];
}

fn get_number(line: &mut &[u8]) -> isize {
    let mut n = 0;
    let mut idx = 0;
    let neg = line[0] == b'-';
    if neg {
        idx += 1
    }
    while idx < line.len() && line[idx].is_ascii_digit() {
        n = n * 10 + (line[idx] - b'0') as isize;
        idx += 1;
    }
    *line = &line[idx..];
    if neg {
        -n
    } else {
        n
    }
}

fn read_values(mut line: &[u8]) -> Box<[isize]> {
    let mut result = Vec::new();
    while !line.is_empty() {
        let n = get_number(&mut line);
        result.push(n);
        eat(&mut line, b' ');
    }
    result.into()
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> isize {
    let mut sum = 0;
    for line in input {
        let line = line.as_ref().as_bytes();
        let mut values = read_values(line);
        let mut last_values = vec![values[0]];
        while values.iter().any(|v| *v != values[0]) {
            let mut new_values = vec![];
            let mut prev_value = values[0];
            for v in &values[1..] {
                let diff = v - prev_value;
                prev_value = *v;
                new_values.push(diff);
            }
            values = new_values.into();
            last_values.push(values[0]);
        }
        let mut v = 0;
        while let Some(nv) = last_values.pop() {
            v = nv - v;
        }
        sum += v;
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
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"
            .lines()
        ),
        2
    );
}
