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

fn count_accepted(
    name: &[u8],
    mut values: [(usize, usize); 4],
    workflows: &HashMap<Box<[u8]>, Vec<Step>>,
) -> usize {
    if *name == b"A"[..] {
        return (values[0].1 - values[0].0 + 1)
            * (values[1].1 - values[1].0 + 1)
            * (values[2].1 - values[2].0 + 1)
            * (values[3].1 - values[3].0 + 1);
    }
    if *name == b"R"[..] {
        return 0;
    }
    let steps = workflows.get(name).unwrap();
    let mut sum = 0;
    for step in steps {
        match step {
            Step::Just(n) => {
                return count_accepted(n, values, workflows) + sum;
            }
            Step::Next(cond) => {
                let (from, to) = values[cond.v as usize];
                if cond.than < from || cond.than > to {
                    continue;
                }
                let mut nval = values;
                if cond.rel == 0 {
                    nval[cond.v as usize].1 = cond.than - 1;
                    values[cond.v as usize].0 = cond.than;
                } else {
                    nval[cond.v as usize].0 = cond.than + 1;
                    values[cond.v as usize].1 = cond.than;
                }
                sum += count_accepted(&cond.dest, nval, workflows);
            }
        }
    }
    unreachable!();
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut workflows = HashMap::new();
    for line in input {
        let mut line = line.as_ref().as_bytes();
        if line.is_empty() {
            break;
        }
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
    count_accepted(
        b"in",
        [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
        &workflows,
    )
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
        167_409_079_868_000
    );
}
