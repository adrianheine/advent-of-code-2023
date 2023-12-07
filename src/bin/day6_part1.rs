use std::io;

fn eat(line: &mut &[u8], c: u8) {
    let mut idx = 0;
    while idx < line.len() && line[idx] == c {
        idx += 1;
    }
    *line = &line[idx..];
}

fn get_number(line: &mut &[u8]) -> usize {
    let mut n = 0;
    let mut idx = 0;
    while idx < line.len() && line[idx].is_ascii_digit() {
        n = n * 10 + (line[idx] - b'0') as usize;
        idx += 1;
    }
    *line = &line[idx..];
    n
}

fn read(mut line: &[u8]) -> Box<[usize]> {
    line = &line[9..];
    eat(&mut line, b' ');
    let mut result = Vec::new();
    while !line.is_empty() {
        let n = get_number(&mut line);
        result.push(n);
        eat(&mut line, b' ');
    }
    result.into()
}

fn calc1(time: usize, distance: usize) -> usize {
    let low =
        time.div_ceil(2) - ((time.div_ceil(2).pow(2) - distance) as f32).sqrt().floor() as usize;
    time - if low * (time - low) <= distance {
        low + 1
    } else {
        low
    } * 2
        + 1
}

fn calc(mut input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let times = read(input.next().unwrap().as_ref().as_bytes());
    let distances = read(input.next().unwrap().as_ref().as_bytes());
    let mut product = 1;
    for (time, distance) in times.iter().zip(distances.iter()) {
        product *= calc1(*time, *distance);
    }
    product
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "Time:      7  15   30
Distance:  9  40  200
"
            .lines()
        ),
        288
    );
}
