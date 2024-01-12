use std::collections::HashMap;

pub fn day19() {
    let str = include_str!("../day19.txt");
    let timer = std::time::Instant::now();
    let total = part1(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> i32 {
    let (workflows, parts) = parse(str);

    parts
        .iter()
        .filter(|part| {
            let mut workflow_name = "in";
            while let Some(workflow) = workflows.get(workflow_name) {
                match workflow.evaluate(part) {
                    State::Accepted => return true,
                    State::NextWorkflow(name) => workflow_name = name,
                    _ => return false,
                }
            }
            panic!();
        })
        .map(|part| part.sum())
        .sum()
}

struct Cond {
    var: u8,
    op: u8,
    value: i32,
}

impl Cond {
    fn is_satisfied(&self, part: &Part) -> bool {
        let value = match self.var {
            b'x' => part.x,
            b'm' => part.m,
            b'a' => part.a,
            b's' => part.s,
            _ => panic!(),
        };
        match self.op {
            b'<' => value < self.value,
            b'>' => value > self.value,
            _ => panic!(),
        }
    }
}

struct Rule<'a> {
    cond: Option<Cond>,
    res: &'a str,
}

#[derive(PartialEq)]
enum State<'a> {
    Accepted,
    Reject,
    NextWorkflow(&'a str),
}
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl Workflow<'_> {
    fn evaluate(&self, part: &Part) -> State {
        for rule in &self.rules {
            if match &rule.cond {
                Some(cond) => cond.is_satisfied(part),
                None => true,
            } {
                return match rule.res {
                    "A" => State::Accepted,
                    "R" => State::Reject,
                    _ => State::NextWorkflow(rule.res),
                };
            }
        }
        panic!();
    }
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

impl Part {
    fn sum(&self) -> i32 {
        self.x + self.m + self.a + self.s
    }
}

fn parse(str: &str) -> (HashMap<&str, Workflow>, Vec<Part>) {
    let (lp, rp) = str.split_once("\n\n").unwrap();

    let workflows = lp
        .lines()
        .map(|workflow| {
            let (name, rest) = workflow
                .split_once('{')
                .map(|(l, r)| (l, r.trim_end_matches('}')))
                .unwrap();
            let rules = rest
                .split(',')
                .map(|rule| {
                    if let Some((cond, res)) = rule.split_once(':') {
                        Rule {
                            cond: Some(Cond {
                                var: cond.as_bytes()[0],
                                op: cond.as_bytes()[1],
                                value: cond[2..].parse().unwrap(),
                            }),
                            res,
                        }
                    } else {
                        Rule {
                            cond: None,
                            res: rule,
                        }
                    }
                })
                .collect();
            (name, Workflow { name, rules })
        })
        .collect::<HashMap<_, _>>();
    let parts = rp
        .lines()
        .map(|part| {
            let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
            part.trim_matches(&['{', '}']).split(',').for_each(|var| {
                let (k, v) = var
                    .split_once('=')
                    .map(|(k, v)| (k.as_bytes()[0], v.parse().unwrap()))
                    .unwrap();
                match k {
                    b'x' => x = v,
                    b'm' => m = v,
                    b'a' => a = v,
                    b's' => s = v,
                    _ => panic!(),
                }
            });
            Part { x, m, a, s }
        })
        .collect::<Vec<_>>();

    (workflows, parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day19_1() {
        let str = "px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(part1(str), 19114);
    }

    #[test]
    fn test_day19_2() {
        let str = include_str!("../day19.txt");
        assert_eq!(part1(str), 432427);
    }
}
