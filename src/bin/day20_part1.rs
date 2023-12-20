use std::collections::HashMap;
use std::collections::VecDeque;
use std::io;
use std::rc::Rc;

fn read_name(line: &mut &[u8]) -> Rc<[u8]> {
    let mut idx = 0;
    while idx < line.len() && line[idx] != b' ' && line[idx] != b',' {
        idx += 1;
    }
    let name = &line[0..idx];
    *line = &line[idx..];
    name.into()
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Pulse {
    Low,
    High,
}

enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<Rc<[u8]>, Pulse>),
    Other,
}

fn calc(input: impl Iterator<Item = impl AsRef<str>>) -> usize {
    let mut modules = HashMap::new();
    let mut unconnected: HashMap<Rc<[u8]>, Vec<Rc<[u8]>>> = HashMap::new();
    for line in input {
        let mut line = line.as_ref().as_bytes();
        let mut module = match line[0] {
            b'%' => {
                let r = Module::FlipFlop(false);
                line = &line[1..];
                r
            }
            b'&' => {
                let r = Module::Conjunction(HashMap::new());
                line = &line[1..];
                r
            }
            _ => Module::Other,
        };
        let name = read_name(&mut line);
        if let Some(inputs) = unconnected.remove(&name) {
            if let Module::Conjunction(x) = &mut module {
                *x = inputs.into_iter().map(|n| (n, Pulse::Low)).collect();
            }
        }
        line = &line[2..];
        let mut outputs = vec![];
        while !line.is_empty() {
            line = &line[2..];
            let output = read_name(&mut line);
            if let Some((_, module)) = modules.get_mut(&output) {
                if let Module::Conjunction(state) = module {
                    state.insert(Rc::clone(&name), Pulse::Low);
                }
            } else {
                unconnected
                    .entry(Rc::clone(&output))
                    .or_default()
                    .push(Rc::clone(&name));
            }
            outputs.push(output);
        }
        modules.insert(name, (outputs, module));
    }
    let mut low_high = (0, 0);
    for _ in 0..1000 {
        let mut pulses: VecDeque<(Rc<[u8]>, _, Rc<[u8]>)> = VecDeque::from([(
            (&b"broadcaster"[..]).into(),
            Pulse::Low,
            ((&b"button"[..]).into()),
        )]);
        while let Some((target, pulse, from)) = pulses.pop_front() {
            *(match pulse {
                Pulse::Low => &mut low_high.0,
                Pulse::High => &mut low_high.1,
            }) += 1;
            match &mut modules.get_mut(&target) {
                Some((outputs, Module::FlipFlop(state))) => {
                    if pulse == Pulse::Low {
                        *state = !*state;
                        let new_pulse = if *state { Pulse::High } else { Pulse::Low };
                        for output in &*outputs {
                            pulses.push_back((Rc::clone(output), new_pulse, Rc::clone(&target)));
                        }
                    }
                }
                Some((outputs, Module::Conjunction(mem))) => {
                    mem.insert(Rc::clone(&from), pulse);
                    let new_pulse = if mem.iter().all(|(_, p)| *p == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };
                    for output in &*outputs {
                        pulses.push_back((Rc::clone(output), new_pulse, Rc::clone(&target)));
                    }
                }
                Some((outputs, Module::Other)) => {
                    for output in &*outputs {
                        pulses.push_back((Rc::clone(output), pulse, Rc::clone(&target)));
                    }
                }
                None => {}
            }
        }
    }
    low_high.0 * low_high.1
}

fn main() {
    println!("{}", calc(io::stdin().lines().map(Result::unwrap)));
}

#[test]
fn test1() {
    assert_eq!(
        calc(
            "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"
            .lines()
        ),
        32_000_000
    );
}

#[test]
fn test2() {
    assert_eq!(
        calc(
            "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"
            .lines()
        ),
        11_687_500
    );
}
