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

fn read_values(mut line: &[u8]) -> Box<[usize]> {
    let mut result = Vec::new();
    while !line.is_empty() {
        let n = get_number(&mut line);
        result.push(n);
        eat(&mut line, b',');
    }
    result.into()
}

fn do_spans(target_spans: &[usize], map_spans: &[(u8, usize)]) -> Option<usize> {
    do_spans_inner(0, target_spans, map_spans)
}

fn do_spans_inner(cur: usize, target_spans: &[usize], map_spans: &[(u8, usize)]) -> Option<usize> {
    //eprintln!("{cur:?} {map_spans:?} {target_spans:?} (");
    if target_spans.is_empty() {
        return if map_spans.iter().all(|(c, _)| *c != b'#') {
            Some(1)
        } else {
            None
        };
    }
    if map_spans.is_empty() {
        return if target_spans.len() == 1 && cur == target_spans[0] {
            Some(1)
        } else {
            None
        };
    }
    match map_spans[0] {
        (b'.', _) => {
            if cur > 0 {
                if cur == target_spans[0] {
                    do_spans_inner(0, &target_spans[1..], &map_spans[1..])
                } else {
                    None
                }
            } else {
                do_spans_inner(0, target_spans, &map_spans[1..])
            }
        }
        (b'#', count) => do_spans_inner(cur + count, target_spans, &map_spans[1..]),
        (b'?', count) => {
            let mut sum = 0;
            let mut new_map_spans = Vec::with_capacity(map_spans.len() + 2);
            new_map_spans.push((b'.', 1));
            if count > 1 {
                new_map_spans.push((b'?', count - 1));
            }
            new_map_spans.extend_from_slice(&map_spans[1..]);
            sum += do_spans_inner(cur, target_spans, &new_map_spans).unwrap_or_default();
            new_map_spans[0].0 = b'#';
            sum += do_spans_inner(cur, target_spans, &new_map_spans).unwrap_or_default();
            if sum > 0 {
                Some(sum)
            } else {
                None
            }
        }
        _ => unreachable!(),
    }
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut sum = 0;
    for line in input {
        let line = line.as_ref().as_bytes();
        let mut idx = 0;
        let mut map_spans: Vec<(u8, usize)> = vec![];
        while line[idx] != b' ' {
            if let Some(span) = map_spans.last_mut() {
                if span.0 == line[idx] {
                    span.1 += 1;
                } else {
                    map_spans.push((line[idx], 1));
                }
            } else {
                map_spans.push((line[idx], 1));
            }
            idx += 1;
        }
        let damaged_spans = read_values(&line[idx + 1..]);
        //eprintln!("{map_spans:?} {damaged_spans:?} {sum}");
        sum += do_spans(&damaged_spans, &map_spans).unwrap();
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
            "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
"
            .lines()
        ),
        21
    );
}
