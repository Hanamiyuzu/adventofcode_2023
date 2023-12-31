use std::collections::HashMap;

pub fn day15() {
    let str = include_str!("../day15.txt");
    let timer = std::time::Instant::now();
    let total = part2(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> i32 {
    str.split(',').map(|s| hash(s)).sum()
}

fn part2(str: &str) -> i32 {
    let mut bucket: HashMap<i32, Vec<(&str, i32)>> = HashMap::new();
    str.split(',').for_each(|s| {
        if s.chars().last().unwrap() == '-' {
            let ts = &s[..s.len() - 1];
            if let Some(sbox) = bucket.get_mut(&hash(ts)) {
                sbox.retain(|&x| x.0 != ts);
            }
        } else if let Some((ts, tv)) = s
            .split_once('=')
            .map(|(a, b)| (a, b.parse::<i32>().unwrap()))
        {
            bucket
                .entry(hash(ts))
                .and_modify(|sbox| match sbox.iter_mut().find(|(ss, _)| ss == &ts) {
                    Some(x) => x.1 = tv,
                    _ => sbox.push((ts, tv)),
                })
                .or_insert(vec![(ts, tv)]);
        } else {
            panic!();
        }
    });
    bucket
        .iter()
        .map(|(k, v)| {
            v.iter()
                .enumerate()
                .map(|(slot, (_, focal_len))| (k + 1) * (slot as i32 + 1) * focal_len)
                .sum::<i32>()
        })
        .sum()
}

fn hash(s: &str) -> i32 {
    s.chars().fold(0, |hash, c| (hash + c as i32) * 17 % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15_1() {
        let str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(str), 1320);
        assert_eq!(part2(str), 145);
    }

    #[test]
    fn test_day15_2() {
        let str = include_str!("../day15.txt");
        assert_eq!(part1(str), 515495);
        assert_eq!(part2(str), 229349);
    }
}
