pub fn day12() {
    let str = include_str!("../day12.txt");

    let total = part1(str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i32 {
    parse(str)
        .into_iter()
        .map(|(mut pattern, counts)| backtrack(&mut pattern, &counts))
        .sum()
}

fn parse(str: &str) -> Vec<(String, Vec<i32>)> {
    str.lines()
        .filter_map(|l| {
            l.split_once(' ').map(|u| {
                (
                    u.0.to_string(),
                    u.1.split(',').map(|n| n.parse().unwrap()).collect(),
                )
            })
        })
        .collect()
}

fn is_valid(pattern: &str, counts: &[i32]) -> bool {
    pattern
        .split('.')
        .filter(|x| !x.is_empty())
        .map(|x| x.chars().count() as i32)
        .collect::<Vec<_>>()
        == counts
}

fn backtrack(pattern: &mut String, counts: &[i32]) -> i32 {
    match pattern.find('?') {
        Some(i) => {
            unsafe {
                pattern.as_bytes_mut()[i] = b'#';
            }
            let a = backtrack(pattern, counts);
            unsafe {
                pattern.as_bytes_mut()[i] = b'.';
            }
            let b = backtrack(pattern, counts);
            unsafe {
                pattern.as_bytes_mut()[i] = b'?';
            }
            a + b
        }
        None => is_valid(pattern, counts) as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part1(str), 21);
    }
}
