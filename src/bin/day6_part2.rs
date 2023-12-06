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
    let mut low = 1;
    let mut high = time.div_ceil(2);
    if high * high < distance {
        return 0;
    }
    while high > low + 1 {
        let mid = (high - low) / 2 + low;
        if mid * (time - mid) <= distance {
            low = mid;
        } else {
            high = mid;
        }
    }
    low = if low * (time - low) <= distance {
        high
    } else {
        low
    };
    time - low * 2 + 1
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
