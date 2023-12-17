use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io;

fn opposite(dir: u8) -> u8 {
    match dir {
        1 => 4,
        2 => 8,
        4 => 1,
        8 => 2,
        _ => unreachable!(),
    }
}

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
    //history: Vec<(usize, usize)>,
}

impl Path {
    fn cost_per_field(&self) -> usize {
        (self.sum * 1000) / (self.pos.0 + self.pos.1 + 1).pow(2)
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

fn maybe_do_step(
    pos: (usize, usize),
    bounds: (usize, usize),
    dir: u8,
) -> Option<((usize, usize), u8)> {
    let v = vec(dir);
    if (pos.0 > 0 || v.0 >= 0)
        && (pos.1 > 0 || v.1 >= 0)
        && (pos.0 < bounds.0 - 1 || v.0 <= 0)
        && (pos.1 < bounds.1 - 1 || v.1 <= 0)
    {
        Some((
            (
                (pos.0 as isize + v.0 as isize) as usize,
                (pos.1 as isize + v.1 as isize) as usize,
            ),
            opposite(dir),
        ))
    } else {
        None
    }
}

const STRAIGHT: usize = 10;

fn maybe_next_path(
    path: &Path,
    out_dir: u8,
    fields: &[Vec<u8>],
    quickest: &mut [[usize; STRAIGHT]],
) -> Option<Path> {
    let bounds = (fields.len(), fields[0].len());
    let (pos, dir) = maybe_do_step(path.pos, bounds, out_dir)?;
    let sum = path.sum + (fields[pos.0][pos.1] as usize);
    let q = &mut quickest[(pos.0 + pos.1 * bounds.0) * 4
        + match dir {
            1 => 1,
            2 => 2,
            4 => 3,
            8 => 0,
            _ => unreachable!(),
        }];
    let new_straight = if path.in_dir == dir {
        path.straight + 1
    } else {
        0
    };
    if new_straight >= 10 {
        return None;
    } else if new_straight == 0 && path.straight < 3 {
        return None;
    }
    if q[new_straight as usize] != 0 && q[new_straight as usize] <= sum {
        return None;
    }
    /*
    for i in 0..=new_straight as usize {
        if q[i] != 0 && q[i] <= sum {
            return None;
        }
    }
    */
    q[new_straight as usize] = sum;
    //let mut history = path.history.clone();
    //history.push(pos);
    Some(Path {
        pos,
        in_dir: dir,
        sum,
        straight: new_straight,
        //history,
    })
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut fields = vec![];
    for (y, line) in input.enumerate() {
        let line = line.as_ref().as_bytes();
        fields.push(vec![]);
        for (_x, c) in line.iter().enumerate() {
            let field = *c - b'0';
            fields[y].push(field);
        }
    }
    let bounds = (fields.len(), fields[0].len());
    let mut min = usize::MAX;
    let mut paths_to_take = BinaryHeap::from([
        Path {
            pos: (0, 1),
            in_dir: 8,
            sum: fields[0][1] as usize,
            straight: 0,
            //history: vec![],
        },
        Path {
            pos: (1, 0),
            in_dir: 1,
            sum: fields[1][0] as usize,
            straight: 0,
            //history: vec![],
        },
    ]);
    let mut quickest = vec![[0; STRAIGHT]; bounds.0 * bounds.1 * 4];
    while let Some(path) = paths_to_take.pop() {
        if path.sum >= min {
            continue;
        }
        if path.pos == (bounds.0 - 1, bounds.1 - 1) && path.straight >= 3 {
            min = path.sum;
            eprintln!("RESULT: {min}");
            /*
            if min <= 105 {
              //eprintln!("{quickest:?}");
                for (y, row) in fields.iter().enumerate() {
                    for (x, field) in row.iter().enumerate() {
                        let e = 27 as char;

                        if path.history.contains(&(y, x)) {
                            eprint!("{e}[0;31m");
                        }
                        eprint!("{}", (*field + b'0') as char);
                        if path.history.contains(&(y, x)) {
                            eprint!("{e}[0m");
                        }
                    }
                    eprintln!();
                }
                eprintln!();
            }
            */
            continue;
        }
        if let Some(p) = maybe_next_path(&path, opposite(path.in_dir), &fields, &mut quickest) {
            paths_to_take.push(p);
        }
        if let Some(p) = maybe_next_path(
            &path,
            if path.in_dir == 1 {
                8
            } else {
                path.in_dir >> 1
            },
            &fields,
            &mut quickest,
        ) {
            paths_to_take.push(p);
        }
        if let Some(p) = maybe_next_path(
            &path,
            if path.in_dir == 8 {
                1
            } else {
                path.in_dir << 1
            },
            &fields,
            &mut quickest,
        ) {
            paths_to_take.push(p);
        }
    }
    /*
    if false {
        for (fields_row, tags_row) in fields.iter().zip(field_tags.iter()) {
            for (field, tag) in fields_row.iter().zip(tags_row.iter()) {
                let e = 27 as char;

                if *tag > 0 {
                    eprint!("{e}[0;31m");
                }
                eprint!("{}", *field as char);
                if *tag > 0 {
                    eprint!("{e}[0m");
                }
            }
            eprintln!();
        }
        eprintln!();
    }
    */
    min
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test_1() {
    assert_eq!(
        calc(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"
            .lines()
        ),
        94
    );
}

#[test]
fn test_3() {
    assert_eq!(
        calc(
            "111111111111
999999999991
999999999991
999999999991
999999999991
"
            .lines()
        ),
        71
    );
}
