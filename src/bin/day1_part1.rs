use std::io::{self, Read};

fn calc(input: impl Iterator<Item = u8>) -> usize {
    let mut sum: usize = 0;
    let mut last: Option<u8> = None;
    for c in input {
        if c >= b'0' && c <= b'9' {
            let v = c - b'0';
            if last.is_none() {
                sum = sum + (10 * v as usize);
            }
            last = Some(v);
        } else if c == b'\n' || c == b'\r' {
            sum = sum + last.unwrap() as usize;
            last = None;
        }
    }
    sum
}

fn main() {
    println!("{}", calc(io::stdin().lock().bytes().map(|v| v.unwrap())))
}
