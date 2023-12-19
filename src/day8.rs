use num_integer::Integer;
use std::collections::HashMap;

pub fn day8() {
    let str = include_str!("../day8.txt");

    let total = part2(str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i32 {
    let (route, rules) = parse(str);

    let mut count = 0;
    let mut pos = rules.get_key_value("AAA").unwrap();
    while pos.0 != &"ZZZ" {
        route.chars().for_each(|c| match c {
            'L' => pos = rules.get_key_value(pos.1 .0).unwrap(),
            'R' => pos = rules.get_key_value(pos.1 .1).unwrap(),
            _ => (),
        });
        count += 1;
    }
    count * route.len() as i32
}

// When I realized that part2 cannot be solved by brute force,
// I tried to find the pattern of the input text and found that for each starting point A,
// there is only one unique endpoint Z.
fn part2(str: &str) -> u64 {
    let (route, rules) = parse(str);

    let mut vpos = rules
        .iter()
        .filter(|x| x.0.ends_with('A'))
        .collect::<Vec<_>>();
    let mut counts = vec![0; vpos.len()];
    vpos.iter_mut().enumerate().for_each(|(i, pos)| {
        while !pos.0.ends_with('Z') {
            route.chars().for_each(|c| match c {
                'L' => *pos = rules.get_key_value(pos.1 .0).unwrap(),
                'R' => *pos = rules.get_key_value(pos.1 .1).unwrap(),
                _ => (),
            });
            counts[i] += 1;
        }
    });
    // println!("counts: {:?}", counts);
    counts.iter().fold(1, |acc, &count| acc.lcm(&count)) * route.len() as u64
}

fn parse(str: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (route, rules) = str.split_once("\n\n").unwrap();

    let mut res = HashMap::new();
    rules.split('\n').for_each(|s| {
        res.insert(&s[..3], (&s[7..10], &s[12..15]));
    });
    (route, res)
}
