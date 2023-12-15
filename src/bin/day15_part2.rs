use std::io;

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut hash = 0;
    let mut boxes: Vec<Vec<(_, u8)>> = vec![vec![]; 256];
    let mut label = vec![];
    let mut op = None;
    for line in input {
        for c in line.as_ref().bytes() {
            match c {
                b',' => {
                    let maybe_idx = boxes[hash].iter().position(|(l, _)| *l == label);
                    if matches!(op, Some(b'-')) {
                        if let Some(idx) = maybe_idx {
                            boxes[hash].remove(idx);
                        }
                    } else if let Some(idx) = maybe_idx {
                        boxes[hash][idx].1 = op.unwrap() - b'0';
                    } else {
                        boxes[hash].push((std::mem::take(&mut label), op.unwrap() - b'0'));
                    }
                    hash = 0;
                    label = vec![];
                }
                b'=' | b'-' => op = Some(c),
                c => {
                    if matches!(op, Some(b'=')) {
                        op = Some(c);
                    } else {
                        label.push(c);
                        hash = ((hash + c as usize) * 17) % 256;
                    }
                }
            }
        }
    }
    let maybe_idx = boxes[hash].iter().position(|(l, _)| *l == label);
    if matches!(op, Some(b'-')) {
        if let Some(idx) = maybe_idx {
            boxes[hash].remove(idx);
        }
    } else if let Some(idx) = maybe_idx {
        boxes[hash][idx].1 = op.unwrap() - b'0';
    } else {
        boxes[hash].push((std::mem::take(&mut label), op.unwrap() - b'0'));
    }

    let mut sum = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (n, (_, lens)) in b.iter().enumerate() {
            sum += (i + 1) * (n + 1) * (*lens as usize);
        }
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
            "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
"
            .lines()
        ),
        145
    );
}
