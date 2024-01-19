use std::collections::{HashMap, VecDeque};

pub fn day20() {
    let str: &'static str = include_str!("../day20.txt");
    let timer = std::time::Instant::now();
    let total = part1(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &'static str) -> i64 {
    unreachable!(); // TODO
    let mut modules = parse(str);
    for module in modules.iter_mut() {
        if let Module::Conjunction(_, map, ts) = module.1 {
            for t in ts {
                map.insert(t, Pulse::Lo);
            }
            map.insert(module.0, Pulse::Lo);
        }
    }
    let (mut hc, mut lc) = (0, 1);
    for _ in 0..1 {
        let mut q = VecDeque::from([("broadcaster", Pulse::Lo)]);
        while let Some((n, pulse)) = q.pop_front() {
            let module = modules.get_mut(n).unwrap();
            for t in module.t() {
                match pulse {
                    Pulse::Lo => lc += 1,
                    Pulse::Hi => hc += 1,
                }
                if t == "output" {
                    continue;
                }
                if !match module {
                    Module::Flipflop(..) => pulse == Pulse::Lo,
                    _ => true,
                } {
                    continue;
                }
                let np = match module {
                    Module::Broadcaster(p, ..) => {
                        *p = pulse;
                        *p
                    }
                    Module::Flipflop(p, ..) => {
                        if pulse == Pulse::Lo {
                            *p = !*p;
                        }
                        *p
                    }
                    Module::Conjunction(p, map, ..) => {
                        map.insert(n, pulse);
                        *p = if map.values().all(|&p| p == Pulse::Hi) {
                            Pulse::Lo
                        } else {
                            Pulse::Hi
                        };
                        *p
                    }
                    _ => panic!(),
                };
                q.push_back((t, np));
            }
            println!("{:?}", modules);
        }
    }
    hc * lc
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Pulse {
    Hi,
    Lo,
}

impl std::ops::Not for Pulse {
    type Output = Pulse;
    fn not(self) -> Pulse {
        match self {
            Pulse::Lo => Pulse::Hi,
            Pulse::Hi => Pulse::Lo,
        }
    }
}

#[derive(Debug)]
enum Module {
    Output,
    Broadcaster(Pulse, Vec<&'static str>),
    Flipflop(Pulse, Vec<&'static str>),
    Conjunction(Pulse, HashMap<&'static str, Pulse>, Vec<&'static str>),
}

impl Module {
    fn t(&self) -> Vec<&'static str> {
        match &self {
            Module::Output => vec![],
            Module::Broadcaster(.., t) => t.clone(),
            Module::Flipflop(.., t) => t.clone(),
            Module::Conjunction(.., t) => t.clone(),
        }
    }
}

fn parse(str: &'static str) -> HashMap<&str, Module> {
    let mut modules = str
        .lines()
        .map(|s| {
            s.split_once(" -> ")
                .map(|(n, t)| {
                    if n == "broadcaster" {
                        (n, Module::Broadcaster(Pulse::Lo, t.split(", ").collect()))
                    } else if let Some(n) = n.strip_prefix('%') {
                        (n, Module::Flipflop(Pulse::Lo, t.split(", ").collect()))
                    } else if let Some(n) = n.strip_prefix('&') {
                        (
                            n,
                            Module::Conjunction(Pulse::Lo, HashMap::new(), t.split(", ").collect()),
                        )
                    } else {
                        panic!();
                    }
                })
                .unwrap()
        })
        .collect::<HashMap<_, _>>();
    modules.insert("output", Module::Output);
    modules
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20_1() {
        let str: &'static str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(part1(str), 32000000);
    }
}
