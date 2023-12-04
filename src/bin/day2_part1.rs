use std::io;

fn get_number(mut input: &[u8]) -> (usize, &[u8]) {
    let mut n: usize = 0;
    while !input.is_empty() && input[0].is_ascii_digit() {
        n = n * 10 + (input[0] - b'0') as usize;
        input = &input[1..];
    }
    (n, input)
}

const MAX_RGB: (usize, usize, usize) = (12, 13, 14);

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum: usize = 0;
    'lines: for line in input {
        let mut line = line.as_ref().as_bytes();
        assert!(line.starts_with(b"Game "));
        let id;
        (id, line) = get_number(&line[5..]);
        line = line.strip_prefix(&[b':']).unwrap();
        while !line.is_empty() {
            line = line.strip_prefix(&[b' ']).unwrap();
            let count;
            (count, line) = get_number(line);
            line = line.strip_prefix(&[b' ']).unwrap();
            if let Some(l) = line.strip_prefix(b"red") {
                line = l;
                if count > MAX_RGB.0 {
                    continue 'lines;
                }
            } else if let Some(l) = line.strip_prefix(b"green") {
                line = l;
                if count > MAX_RGB.1 {
                    continue 'lines;
                }
            } else if let Some(l) = line.strip_prefix(b"blue") {
                line = l;
                if count > MAX_RGB.2 {
                    continue 'lines;
                }
            } else {
                panic!("what {:?}", std::str::from_utf8(line).unwrap());
            }
            if !line.is_empty() {
                line = &line[1..];
            } // ; or ,
        }
        sum += id;
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
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"
            .lines()
        ),
        8
    );
}
