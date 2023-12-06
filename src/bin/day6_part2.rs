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

fn calc(mut input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let time = read(input.next().unwrap().as_ref().as_bytes());
    let distance = read(input.next().unwrap().as_ref().as_bytes());
    let mut counts = 0;
    for i in 0..=time {
        if (time - i) * i > distance {
            counts += 1;
        } else if counts > 0 {
            break;
        }
    }
    counts
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
