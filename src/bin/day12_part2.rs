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

fn get_biggest_middle(ch: u8, map_spans: &[(u8, usize)]) -> Option<(usize, usize)> {
    let (size, items) = map_spans
        .iter()
        .enumerate()
        .filter(|(_, (c, _))| *c == ch)
        .fold((0, vec![]), |(max, mut items), (idx, (_, l))| {
            if *l > max {
                (*l, vec![idx])
            } else {
                if *l == max {
                    items.push(idx);
                };
                (max, items)
            }
        });
    if size == 0 {
        return None;
    }
    let idx = items[items.len() / 2];
    Some((size, idx))
}

fn do_spans(target_spans: &[usize], map_spans: &[(u8, usize)]) -> u64 {
    if let Some((size, idx)) = get_biggest_middle(b'#', map_spans) {
        //eprintln!("{map_spans:?} {target_spans:?} ({idx} {size}) (");
        let mut sum = 0;
        for (target_idx, target_span) in target_spans.iter().enumerate() {
            //eprintln!("{target_span} {size}");
            if *target_span == size {
                let mut new_map_spans = Vec::from(&map_spans[0..idx]);
                if idx > 0 && map_spans[idx - 1].0 == b'?' {
                    let left = new_map_spans.last().unwrap().1 - 1;
                    if left > 0 {
                        new_map_spans.last_mut().unwrap().1 = left;
                        new_map_spans.push((0, 0));
                    }
                    *new_map_spans.last_mut().unwrap() = (b'.', 1);
                }
                let front = do_spans(&target_spans[0..target_idx], &new_map_spans);
                if front > 0 {
                    let new_map_spans = if idx + 1 < map_spans.len() && map_spans[idx + 1].0 == b'?'
                    {
                        if map_spans[idx + 1].1 == 1 {
                            Vec::from(&map_spans[idx + 2..])
                        } else {
                            let mut new_map_spans = Vec::from(&map_spans[idx + 1..]);
                            new_map_spans[0].1 -= 1;
                            new_map_spans
                        }
                    } else {
                        Vec::from(&map_spans[idx + 1..])
                    };
                    let v = do_spans(&target_spans[target_idx + 1..], &new_map_spans);
                    if v == 0 {
                        //break;
                    } // FIXME This could actually be a binsearch to head, too
                    sum += front * v;
                }
            } else if *target_span > size {
                let mut new_targets = Vec::from(&target_spans[0..=target_idx]);
                let diff = target_span - size;
                let mut any_before = false;
                let mut any_after = false;
                let max_from_front = if idx == 0 || map_spans[idx - 1].0 == b'.' {
                    0
                } else {
                    diff
                };
                'from_front: for from_front in 0..=max_from_front {
                    //eprintln!("from_front {from_front}");
                    *new_targets.last_mut().unwrap() = from_front;
                    let mut new_map_spans = Vec::from(&map_spans[0..idx]);
                    let mut left = from_front;
                    while left > 0 {
                        let Some(&(c, l)) = new_map_spans.last() else {
                            continue 'from_front;
                        };
                        if c == b'.' {
                            continue 'from_front;
                        }
                        left = if left > l {
                            new_map_spans.pop();
                            left - l
                        } else if left == l {
                            new_map_spans.pop();
                            0
                        } else {
                            match new_map_spans.last_mut() {
                                Some((b'?', l)) => {
                                    *l -= left;
                                }
                                Some((b'#', _)) => continue 'from_front,
                                Some((b'.', _)) | None => unreachable!(),
                                _ => unreachable!(),
                            }
                            0
                        }
                    }
                    match new_map_spans.last_mut() {
                        Some((b'?', l)) => {
                            *l -= 1;
                            if *l == 0 {
                                new_map_spans.pop();
                            }
                            new_map_spans.push((b'.', 1));
                        }
                        Some((b'#', _)) => continue 'from_front,
                        Some((b'.', _)) | None => {}
                        _ => unreachable!(),
                    }
                    if from_front > 0 {
                        new_map_spans.push((b'#', from_front));
                    };
                    let before = do_spans(
                        &new_targets[0..(new_targets.len() - if from_front == 0 { 1 } else { 0 })],
                        &new_map_spans,
                    );
                    if before > 0 {
                        let after = do_spans_inner(
                            size + from_front,
                            &target_spans[target_idx..],
                            &map_spans[idx + 1..],
                        );
                        if after > 0 {
                            sum += before * after;
                            any_after = true;
                        }
                        any_before = true;
                    }
                }
                if !any_after && any_before {
                    //break;
                } // FIXME This could actually be a binsearch to head, too
            }
        }
        if sum > 0 {
            //eprintln!("{map_spans:?} {target_spans:?} ({idx} {size}) {sum}");
        }
        //eprintln!(") {sum}");
        sum
    } else if let Some((_, idx)) = get_biggest_middle(b'.', map_spans) {
        if target_spans.is_empty() {
            return 1;
        }

        /*
          eprintln!(
              "{} {target_spans:?} {map_spans:?}[ {idx} ] (",
              vec![" "; 80 - map_spans.len() - 9].join("")
          );
        */
        let mut sum = 0;
        for target_idx in 0..=target_spans.len() {
            /*
              eprintln!("IDX: {target_idx}");
              eprintln!(
                  "({:?} {:?}) ({:?} {:?})",
                  &target_spans[0..target_idx],
                  &map_spans[0..idx],
                  &target_spans[target_idx..],
                  &map_spans[idx + 1..],
              );
            */
            let before = do_spans(&target_spans[0..target_idx], &map_spans[0..idx]);
            if before > 0 {
                let after = do_spans(&target_spans[target_idx..], &map_spans[idx + 1..]);
                sum += before * after;
            }
        }
        //eprintln!("{} ) {sum}", vec![" "; 80 - map_spans.len() - 9].join(""));
        return sum;
    } else {
        return do_spans_inner(0, target_spans, map_spans);
    }
}

fn how_many(size: usize, n: usize) -> usize {
    if n == 1 {
        size
    } else if n * 2 - 1 == size {
        1
    } else {
        let mut sum = 0;
        for s in 1..size - 1 {
            sum += how_many(s, n - 1);
        }
        sum
    }
}

fn do_spans_inner(cur: usize, target_spans: &[usize], map_spans: &[(u8, usize)]) -> u64 {
    //eprintln!("{cur:?} {map_spans:?} {target_spans:?} [");
    if target_spans.is_empty() {
        return if map_spans.iter().all(|(c, _)| *c != b'#') {
            1
        } else {
            0
        };
    }
    if map_spans.is_empty() {
        return if target_spans.len() == 1 && cur == target_spans[0] {
            1
        } else {
            0
        };
    }
    match map_spans[0] {
        (b'.', _) => {
            if cur > 0 {
                if cur == target_spans[0] {
                    do_spans(&target_spans[1..], &map_spans[1..])
                } else {
                    0
                }
            } else {
                do_spans(target_spans, &map_spans[1..])
            }
        }
        (b'#', c) => do_spans_inner(cur + c, target_spans, &map_spans[1..]),
        (b'?', count) => {
            if cur > 0 {
                if cur >= target_spans[0] {
                    let mut new_map_spans = Vec::with_capacity(map_spans.len() + 2);
                    new_map_spans.push((b'.', 1));
                    if count > 1 {
                        new_map_spans.push((b'?', count - 1));
                    }
                    new_map_spans.extend_from_slice(&map_spans[1..]);
                    do_spans_inner(cur, target_spans, &new_map_spans)
                } else if count >= target_spans[0] - cur {
                    let mut new_map_spans = Vec::with_capacity(map_spans.len() + 2);
                    new_map_spans.push((b'#', target_spans[0] - cur));
                    let left = count - (target_spans[0] - cur);
                    if left > 0 {
                        new_map_spans.push((b'?', left));
                    }
                    new_map_spans.extend_from_slice(&map_spans[1..]);
                    do_spans_inner(cur, target_spans, &new_map_spans)
                } else {
                    // FIXME: old algorithm
                    let mut sum = 0;
                    let mut new_map_spans = Vec::with_capacity(map_spans.len() + 2);
                    new_map_spans.push((b'.', 1));
                    if count > 1 {
                        new_map_spans.push((b'?', count - 1));
                    }
                    new_map_spans.extend_from_slice(&map_spans[1..]);
                    sum += do_spans_inner(cur, target_spans, &new_map_spans);
                    new_map_spans[0].0 = b'#';
                    sum += do_spans_inner(cur, target_spans, &new_map_spans);
                    sum
                }
            } else if map_spans.len() < 2 || map_spans[1].0 == b'.' {
                let mut size = count;
                let mut n = 0;
                let mut sum = do_spans(target_spans, &map_spans[1..]);
                while n < target_spans.len() && size > target_spans[n] - 1 {
                    size -= target_spans[n] - 1;
                    n += 1;
                    if n * 2 - 1 > size {
                        break;
                    }
                    let inner = do_spans(&target_spans[n..], &map_spans[1..]);
                    if inner > 0 {
                        sum += inner * how_many(size, n) as u64;
                    }
                }
                sum
            } else {
                // FIXME: old algorithm
                let mut sum = 0;
                let mut new_map_spans = Vec::with_capacity(map_spans.len() + 2);
                new_map_spans.push((b'.', 1));
                if count > 1 {
                    new_map_spans.push((b'?', count - 1));
                }
                new_map_spans.extend_from_slice(&map_spans[1..]);
                sum += do_spans(target_spans, &new_map_spans);
                new_map_spans[0].0 = b'#';
                sum += do_spans(target_spans, &new_map_spans);
                sum
            }
        }
        _ => unreachable!(),
    }
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> u64 {
    let mut sum = 0;
    for line in input {
        let line = line.as_ref().as_bytes();
        let mut idx = 0;
        let mut map_spans: Vec<(u8, usize)> = vec![];
        for i in 0..5 {
            idx = 0;
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
            if i < 4 {
                if map_spans.last().unwrap().0 == b'?' {
                    map_spans.last_mut().unwrap().1 += 1;
                } else {
                    map_spans.push((b'?', 1));
                }
            }
        }
        let damaged_spans_raw = read_values(&line[idx + 1..]);
        let mut damaged_spans = Vec::with_capacity(damaged_spans_raw.len() * 5);
        for _ in 0..5 {
            damaged_spans.extend_from_slice(&damaged_spans_raw);
        }
        let v = do_spans(&damaged_spans, &map_spans);
        eprintln!("{map_spans:?} {damaged_spans:?} {v}");
        assert!(v > 0);
        sum += v;
    }
    sum
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test1() {
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
        525_152
    );
}

#[test]
fn test2() {
    assert_eq!(calc("??##???..??????? 6,1".lines()), 9_174_400);
}

#[test]
fn test2_detail() {
    assert_eq!(
        do_spans(&[3, 3], &[(63, 1), (35, 2), (46, 1), (63, 2), (35, 1)]),
        1
    );
}

#[test]
fn test3() {
    assert_eq!(calc(".??..??...?##. 1,1,3".lines()), 16384);
}

#[test]
fn test3_detail() {
    assert_eq!(
        do_spans(
            &[3, 3,],
            &[
                (63, 1),
                (35, 2),
                (46, 1),
                (63, 1),
                (46, 1),
                (63, 1),
                (35, 2),
            ]
        ),
        1
    );
}

#[test]
fn test4() {
    assert_eq!(do_spans(&[2, 3], &[(63, 7), (35, 1), (46, 1)]), 3);
}

#[test]
fn test5() {
    assert_eq!(calc("?.?.?.?.???.?.??.?? 1,1".lines()), 77_852_105_332);
}

#[test]
fn test6() {
    assert_eq!(do_spans(&[2, 3], &[(63, 7), (35, 1)]), 3);
}

#[test]
fn test6_detail() {
    assert_eq!(do_spans(&[2], &[(63, 4)]), 3);
}

#[test]
#[ignore]
fn test7() {
    assert_eq!(calc("?????????? 1,1,2".lines()), 3_247_943_160);
}

#[test]
fn test8() {
    assert_eq!(do_spans(&[2], &[(63, 2)]), 1);
}

#[test]
fn test9() {
    assert_eq!(do_spans(&[1], &[(63, 4)]), 4);
}

#[test]
fn test10() {
    assert_eq!(do_spans(&[1, 1], &[(63, 3)]), 1);
}

#[test]
fn test_how_many() {
    assert_eq!(how_many(1, 1), 1);
    assert_eq!(how_many(2, 1), 2);
    assert_eq!(how_many(3, 1), 3);
    assert_eq!(how_many(3, 2), 1);
    assert_eq!(how_many(4, 1), 4);
    assert_eq!(how_many(4, 2), 3);
    assert_eq!(how_many(5, 1), 5);
    assert_eq!(how_many(5, 2), 6);
    assert_eq!(how_many(5, 3), 1);
    assert_eq!(how_many(6, 1), 6);
    assert_eq!(how_many(6, 2), 10);
    assert_eq!(how_many(6, 3), 4);
    assert_eq!(how_many(7, 1), 7);
    assert_eq!(how_many(7, 2), 15);
    assert_eq!(how_many(7, 3), 10);
    assert_eq!(how_many(7, 4), 1);
    assert_eq!(how_many(8, 1), 8);
    assert_eq!(how_many(8, 2), 21);
    assert_eq!(how_many(8, 3), 20);
    assert_eq!(how_many(8, 4), 5);
}
