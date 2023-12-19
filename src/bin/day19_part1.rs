use std::collections::HashMap;
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

enum Step {
    Just(Box<[u8]>),
    Next(Cond),
}
struct Cond {
    v: u8,   // x, m, a, s
    rel: u8, // <, >
    than: usize,
    dest: Box<[u8]>,
}

fn read_name(line: &mut &[u8]) -> Box<[u8]> {
    let mut idx = 0;
    while line[idx] != b'{' && line[idx] != b',' && line[idx] != b'}' {
        idx += 1;
    }
    let name = &line[0..idx];
    *line = &line[idx..];
    name.into()
}

fn is_accepted(values: [usize; 4], workflows: &HashMap<Box<[u8]>, Vec<Step>>) -> bool {
    let mut name: Box<[u8]> = (*b"in").into();
    'name: loop {
        if *name == b"A"[..] {
            return true;
        }
        if *name == b"R"[..] {
            return false;
        }
        let steps = workflows.get(&name).unwrap();
        for step in steps {
            match step {
                Step::Just(n) => {
                    name = n.clone();
                    continue 'name;
                }
                Step::Next(cond) => {
                    let a = values[cond.v as usize];
                    if (cond.rel == 0 && a < cond.than) || (cond.rel == 1 && a > cond.than) {
                        {
                            name = cond.dest.clone();
                            continue 'name;
                        }
                    }
                }
            }
        }
        unreachable!();
    }
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut workflows = HashMap::new();
    let mut sum = 0;
    let mut reading_parts = false;
    for line in input {
        let mut line = line.as_ref().as_bytes();
        if line.is_empty() {
            reading_parts = true;
            continue;
        }
        if reading_parts {
            line = &line[3..];
            let x = get_number(&mut line);
            line = &line[3..];
            let m = get_number(&mut line);
            line = &line[3..];
            let a = get_number(&mut line);
            line = &line[3..];
            let s = get_number(&mut line);
            if is_accepted([x, m, a, s], &workflows) {
                sum += x + m + a + s;
            }
        } else {
            let name = read_name(&mut line);
            let mut steps = vec![];
            line = &line[1..];
            while !line.is_empty() && line[0] != b'}' {
                let v = if line[1] == b'<' || line[1] == b'>' {
                    match line[0] {
                        b'x' => 0,
                        b'm' => 1,
                        b'a' => 2,
                        b's' => 3,
                        _ => unreachable!(),
                    }
                } else {
                    steps.push(Step::Just(read_name(&mut line)));
                    line = &line[1..];
                    continue;
                };
                line = &line[1..];
                let rel = match line[0] {
                    b'<' => 0,
                    b'>' => 1,
                    _ => unreachable!(),
                };
                line = &line[1..];
                let than = get_number(&mut line);
                line = &line[1..];
                let dest = read_name(&mut line);
                line = &line[1..];
                steps.push(Step::Next(Cond { v, rel, than, dest }));
            }
            workflows.insert(name, steps);
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
            "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"
            .lines()
        ),
        19114
    );
}
