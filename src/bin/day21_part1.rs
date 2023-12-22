use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;

fn vec(dir: u8) -> (i8, i8) {
    match dir {
        1 => (-1, 0),
        2 => (0, 1),
        4 => (1, 0),
        8 => (0, -1),
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Path {
    pos: (usize, usize),
    in_dir: u8,
    straight: u8,
    sum: usize,
}

impl Path {
    fn cost_per_field(&self) -> usize {
        (self.sum * 1000) / (self.pos.0 + self.pos.1 + 1)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost_per_field()
            .cmp(&self.cost_per_field())
            .then_with(|| (self.pos.0 + self.pos.1).cmp(&(other.pos.0 + other.pos.1)))
            .then_with(|| other.straight.cmp(&self.straight))
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn maybe_do_step(pos: (usize, usize), bounds: (usize, usize), dir: u8) -> Option<(usize, usize)> {
    let v = vec(dir);
    if (pos.0 > 0 || v.0 >= 0)
        && (pos.1 > 0 || v.1 >= 0)
        && (pos.0 < bounds.0 - 1 || v.0 <= 0)
        && (pos.1 < bounds.1 - 1 || v.1 <= 0)
    {
        Some((
            (pos.0 as isize + v.0 as isize) as usize,
            (pos.1 as isize + v.1 as isize) as usize,
        ))
    } else {
        None
    }
}

fn calc(steps: usize, input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut fields = vec![];
    let mut positions = HashSet::new();
    for (y, line) in input.enumerate() {
        let line = line.as_ref().as_bytes();
        fields.push(vec![]);
        for (x, c) in line.iter().enumerate() {
            let field = *c != b'#';
            if *c == b'S' {
                positions.insert((y, x));
            }
            fields[y].push(field);
        }
    }
    let bounds = (fields.len(), fields[0].len());
    for _ in 0..steps {
        let mut new_positions = HashSet::new();
        for position in positions {
            for dir in [1, 2, 4, 8] {
                if let Some(new_pos) = maybe_do_step(position, bounds, dir) {
                    if fields[new_pos.0][new_pos.1] {
                        new_positions.insert(new_pos);
                    }
                }
            }
        }
        positions = new_positions;
    }
    positions.len()
}

fn main() {
    println!("{}", calc(64, io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test_1() {
    assert_eq!(
        calc(
            6,
            "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
"
            .lines()
        ),
        16
    );
}
