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

fn read_seeds(mut line: &[u8]) -> Vec<(usize, usize, usize)> {
    line = &line[7..];
    let mut result = Vec::new();
    while !line.is_empty() {
        let start = get_number(&mut line);
        eat(&mut line, b' ');
        let len = get_number(&mut line);
        result.insert(
            result.partition_point(|(v, _, _)| *v < start),
            (start, len, start),
        );
        eat(&mut line, b' ');
    }
    result
}

fn merge(
    value_to_seed: &mut Vec<(usize, usize, usize)>,
    next_value_to_seed: &mut Vec<(usize, usize, usize)>,
) {
    while let Some(n) = next_value_to_seed.pop() {
        value_to_seed.insert(value_to_seed.partition_point(|(v, _, _)| *v < n.0), n);
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
        let src_range_end = src_range_start + range_length;

        let end_of_range_idx =
            value_to_seed.partition_point(|(start, _, _)| *start <= src_range_end);
        let begin_of_range_idx = value_to_seed[..end_of_range_idx]
            .partition_point(|(start, len, _)| start + len < src_range_start);
        let mut unchanged_ranges = Vec::with_capacity(end_of_range_idx - begin_of_range_idx);
        for idx in begin_of_range_idx..end_of_range_idx {
            let (start, len, seed) = value_to_seed[idx];
            let offset_in_range = src_range_start.saturating_sub(start);
            if offset_in_range > 0 {
                unchanged_ranges.push((start, offset_in_range, seed));
            }
            next_value_to_seed.push((
                dest_range_start + start.saturating_sub(src_range_start),
                (src_range_end - start).min(len - offset_in_range),
                seed + offset_in_range,
            ));
            let tail_of_range = (start + len).saturating_sub(src_range_end);
            if tail_of_range > 0 {
                unchanged_ranges.push((src_range_end, tail_of_range, seed + src_range_end - start));
            }
        }
        value_to_seed.splice(
            begin_of_range_idx..end_of_range_idx,
            unchanged_ranges.into_iter(),
        );
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
        46
    );
}
