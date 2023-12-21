pub fn day4() {
    let strs = include_str!("../day4.txt").lines().collect::<Vec<_>>();

    let mut total = 0;
    for str in &strs {
        total += part1(str);
    }
    println!("total: {}", total);
    let total = part2(&strs);
    println!("total: {}", total);
}

fn part1(str: &str) -> i32 {
    match parse_win_count(str) {
        0 => 0,
        count => 1 << (count - 1),
    }
}

fn part2(strs: &Vec<&str>) -> i32 {
    let mut card_counts = vec![1; strs.len()];
    let mut total = 0;
    for i in 0..card_counts.len() {
        total += card_counts[i];
        for j in 0..parse_win_count(&strs[i]) {
            card_counts[i + 1 + j] += card_counts[i];
        }
    }
    total
}

fn parse_win_count(str: &str) -> usize {
    use std::collections::BTreeSet;
    let str_to_nums = |str: &str| {
        str.trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<BTreeSet<_>>()
    };
    let (win_str, have_str) = str.split_once(": ").unwrap().1.split_once('|').unwrap();
    let (win_nums, have_nums) = (str_to_nums(win_str), str_to_nums(have_str));
    win_nums.intersection(&have_nums).count()
}
