use std::collections::{HashMap, VecDeque};

pub fn day20() {
    let str: &'static str = include_str!("../day20.txt");
    let timer = std::time::Instant::now();
    let total = part1(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &'static str) -> i64 {
    let mut modules = parse(str);
    let (mut hc, mut lc) = (0, 1000);
    for _ in 0..1000 {
        let mut q = VecDeque::from([("broadcaster", Pulse::Lo)]);
        while let Some((n, pulse)) = q.pop_front() {
            for t in modules.get(n).unwrap().ts() {
                match pulse {
                    Pulse::Lo => lc += 1,
                    Pulse::Hi => hc += 1,
                }
                if t == "rx" || t == "output" {
                    continue;
                }
                let module = modules.get_mut(t).unwrap();
                if !match module {
                    Module::Flipflop(..) => pulse == Pulse::Lo,
                    _ => true,
                } {
                    continue;
                }
                let np = match module {
                    Module::Broadcaster(..) => pulse,
                    Module::Flipflop(p, ..) => {
                        if pulse == Pulse::Lo {
                            *p = !*p;
                        }
                        *p
                    }
                    Module::Conjunction(map, ..) => {
                        map.insert(n, pulse);
                        if map.values().all(|&p| p == Pulse::Hi) {
                            Pulse::Lo
                        } else {
                            Pulse::Hi
                        }
                    }
                    _ => panic!(),
                };
                q.push_back((t, np));
            }
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

#[derive(Debug, Clone)]
enum Module {
    Output,
    Broadcaster(Vec<&'static str>),
    Flipflop(Pulse, Vec<&'static str>),
    Conjunction(HashMap<&'static str, Pulse>, Vec<&'static str>),
}

impl Module {
    fn ts(&self) -> Vec<&'static str> {
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
                        (n, Module::Broadcaster(t.split(", ").collect()))
                    } else if let Some(n) = n.strip_prefix('%') {
                        (n, Module::Flipflop(Pulse::Lo, t.split(", ").collect()))
                    } else if let Some(n) = n.strip_prefix('&') {
                        (
                            n,
                            Module::Conjunction(HashMap::new(), t.split(", ").collect()),
                        )
                    } else {
                        panic!();
                    }
                })
                .unwrap()
        })
        .collect::<HashMap<_, _>>();
    for (n, module) in modules.clone() {
        for t in module.ts() {
            if let Some(Module::Conjunction(map, _)) = modules.get_mut(t) {
                map.insert(n, Pulse::Lo);
            }
        }
    }
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
