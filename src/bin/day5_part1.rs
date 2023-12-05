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

fn read_seeds(mut line: &[u8]) -> Vec<(usize, usize)> {
    line = &line[7..];
    let mut result = Vec::new();
    while !line.is_empty() {
        let n = get_number(&mut line);
        result.insert(result.partition_point(|(v, _)| *v < n), (n, n));
        eat(&mut line, b' ');
    }
    result
}

fn merge(value_to_seed: &mut Vec<(usize, usize)>, next_value_to_seed: &mut Vec<(usize, usize)>) {
    while let Some(n) = next_value_to_seed.pop() {
        value_to_seed.insert(value_to_seed.partition_point(|(v, _)| *v < n.0), n);
    }
}

fn calc(mut input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut value_to_seed = read_seeds(input.next().unwrap().as_ref().as_bytes());
    let mut expect_caption = false;
    let mut next_value_to_seed = Vec::new();
    for line in input {
        if expect_caption {
            // ignore the name of the map
            merge(&mut value_to_seed, &mut next_value_to_seed);
            expect_caption = false;
            continue;
        }
        let mut line = line.as_ref().as_bytes();
        if line == b"" {
            expect_caption = true;
            continue;
        }
        let dest_range_start = get_number(&mut line);
        eat(&mut line, b' ');
        let src_range_start = get_number(&mut line);
        eat(&mut line, b' ');
        let range_length = get_number(&mut line);
        let start_idx = value_to_seed.partition_point(|(v, _)| *v < src_range_start);
        let mut idx = start_idx;
        while idx < value_to_seed.len() && value_to_seed[idx].0 <= src_range_start + range_length {
            value_to_seed[idx].0 = dest_range_start + value_to_seed[idx].0 - src_range_start;
            idx += 1;
        }
        next_value_to_seed.extend_from_slice(&value_to_seed[start_idx..idx]);
        value_to_seed.drain(start_idx..idx);
    }
    merge(&mut value_to_seed, &mut next_value_to_seed);
    value_to_seed[0].0
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test() {
    assert_eq!(
        calc(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"
            .lines()
        ),
        35
    );
}
