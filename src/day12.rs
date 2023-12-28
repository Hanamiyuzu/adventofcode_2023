use std::{collections::HashMap, time::Instant};

pub fn day12() {
    let str = include_str!("../day12.txt");
    let timer = Instant::now();
    let total = part2(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> u64 {
    parse(str)
        .iter()
        .map(|(pattern, counts)| collect(pattern.as_bytes(), &counts))
        .sum()
}

fn part2(str: &str) -> u64 {
    parse(str)
        .iter()
        .map(|(pattern, counts)| {
            let pattern = vec![*pattern; 5].join("?");
            let counts = (0..5).flat_map(|_| counts.clone()).collect::<Vec<_>>();
            collect(pattern.as_bytes(), &counts)
        })
        .sum()
}

fn parse(str: &str) -> Vec<(&str, Vec<i32>)> {
    str.lines()
        .filter_map(|l| {
            l.split_once(' ')
                .map(|u| (u.0, u.1.split(',').map(|n| n.parse().unwrap()).collect()))
        })
        .collect()
}

fn collect(pattern: &[u8], sizes: &[i32]) -> u64 {
    collect_with_cache(pattern, sizes, 0, 0, 0, &mut HashMap::new())
}

fn collect_with_cache(
    pattern: &[u8],
    sizes: &[i32],
    i: usize,
    j: usize,
    cur_size: i32,
    cache: &mut HashMap<(usize, usize, i32), u64>,
) -> u64 {
    if i >= pattern.len() {
        return match j {
            _ if j >= sizes.len() => 1,
            _ if j == sizes.len() - 1 && sizes[j] == cur_size => 1,
            _ => 0,
        };
    }

    match pattern[i] {
        b'?' => {
            if let Some(&n) = cache.get(&(i, j, cur_size)) {
                return n;
            }
            let mut total = 0;
            if cur_size == 0 {
                total += collect_with_cache(pattern, sizes, i + 1, j, 0, cache);
            }
            if let Some(&size) = sizes.get(j) {
                if cur_size < size {
                    total += collect_with_cache(pattern, sizes, i + 1, j, cur_size + 1, cache);
                } else {
                    total += collect_with_cache(pattern, sizes, i + 1, j + 1, 0, cache);
                }
            }
            cache.insert((i, j, cur_size), total);
            return total;
        }
        b'#' => {
            if j >= sizes.len() || cur_size + 1 > sizes[j] {
                return 0;
            }
            return collect_with_cache(pattern, sizes, i + 1, j, cur_size + 1, cache);
        }
        b'.' => {
            if cur_size == 0 {
                return collect_with_cache(pattern, sizes, i + 1, j, 0, cache);
            }
            if j >= sizes.len() || cur_size != sizes[j] {
                return 0;
            }
            return collect_with_cache(pattern, sizes, i + 1, j + 1, 0, cache);
        }
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_1() {
        let str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(str), 21);
        assert_eq!(part2(str), 525152);
    }

    #[test]
    fn test_day12_2() {
        let str = include_str!("../day12.txt");
        assert_eq!(part1(str), 7705);
        assert_eq!(part2(str), 50338344809230);
    }
}
