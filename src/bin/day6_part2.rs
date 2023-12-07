use std::io;

fn read(line: &[u8]) -> usize {
    let mut n = 0;
    let mut idx = 9;
    while idx < line.len() {
        while idx < line.len() && line[idx] == b' ' {
            idx += 1;
        }
        n = n * 10 + (line[idx] - b'0') as usize;
        idx += 1;
    }
    n
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
    let time = read(input.next().unwrap().as_ref().as_bytes());
    let distance = read(input.next().unwrap().as_ref().as_bytes());
    calc1(time, distance)
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
        71503
    );
}
