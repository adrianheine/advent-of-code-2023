use std::io;

fn eat(line: &mut &[u8], c: u8) {
    let mut idx = 0;
    while line[idx] == c {
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

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum: usize = 0;
    for line in input {
        let mut my_numbers = Vec::with_capacity(10);
        let mut points = 0;
        let mut line = line.as_ref().as_bytes();
        line = &line[line
            .iter()
            .enumerate()
            .find(|(_, v)| **v == b':')
            .unwrap()
            .0
            + 1..];
        eat(&mut line, b' ');
        while line[0] != b'|' {
            my_numbers.push(get_number(&mut line));
            eat(&mut line, b' ');
        }
        line = &line[1..]; // Eat '|'
        while !line.is_empty() {
            eat(&mut line, b' ');
            let n = get_number(&mut line);
            if let Some((idx, _)) = my_numbers.iter().enumerate().find(|(_, v)| **v == n) {
                my_numbers.swap_remove(idx);
                points = if points == 0 { 1 } else { points * 2 };
            }
        }
        sum += points;
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
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
            .lines()
        ),
        13
    );
}
