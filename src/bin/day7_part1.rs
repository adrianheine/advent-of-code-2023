use std::io;

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

fn rank_hand(hand: &[u8]) -> usize {
    let mut distinct = Vec::with_capacity(hand.len());
    'hand: for c in hand {
        for (card, count) in &mut distinct.iter_mut() {
            if *card == c {
                *count += 1;
                continue 'hand;
            }
        }
        distinct.push((c, 1));
    }
    let mut value: usize = match distinct.len() {
        1 => 7,
        2 => {
            if distinct.iter().any(|(_, count)| *count == 4) {
                6
            } else {
                5
            }
        }
        3 => {
            if distinct.iter().any(|(_, count)| *count == 3) {
                4
            } else {
                3
            }
        }
        4 => 2,
        5 => 1,
        _ => unreachable!(),
    };
    for c in hand {
        value = value * 14
            + match *c {
                b'A' => 13,
                b'K' => 12,
                b'Q' => 11,
                b'J' => 10,
                b'T' => 9,
                c => c - b'2',
            } as usize;
    }
    value
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut vs = Vec::new();
    for line in input {
        let line = line.as_ref().as_bytes();
        let rank = rank_hand(&line[0..5]);
        let bid = get_number(&mut &line[6..]);
        vs.insert(vs.partition_point(|(v, _)| *v < rank), (rank, bid));
    }
    vs.into_iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum()
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"
            .lines()
        ),
        6440
    );
}
