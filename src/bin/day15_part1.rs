use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum = 0;
    let mut hash = 0;
    for line in input {
        for c in line.as_ref().bytes() {
            match c {
                b',' => {
                    sum += hash as usize;
                    hash = 0;
                }
                c => hash = (((u16::from(c) + u16::from(hash)) * 17) % 256) as u8,
            }
        }
    }
    sum + hash as usize
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"
            .lines()
        ),
        1320
    );
}
