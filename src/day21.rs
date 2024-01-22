use std::collections::HashSet;

pub fn day21() {
    let str = include_str!("../day21.txt");
    let timer = std::time::Instant::now();
    let total = part1(str, 64);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str, step: i32) -> i32 {
    let (start, map) = parse(str);
    let mut q1 = HashSet::from([(start, step)]);
    let mut q2;
    for _ in (1..=step).rev() {
        q2 = q1;
        q1 = HashSet::new();
        for ((i, j), step) in q2 {
            for (row, col) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if row < map.len() && col < map[0].len() && map[row][col] != '#' {
                    q1.insert(((row, col), step - 1));
                }
            }
        }
    }
    q1.len() as i32
}

fn parse(str: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut start = (0, 0);
    let map = str
        .lines()
        .enumerate()
        .map(|(i, s)| {
            s.chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        start = (i, j);
                    }
                    c
                })
                .collect()
        })
        .collect();
    (start, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day20_1() {
        let str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(part1(str, 6), 16);
    }

    #[test]
    fn test_day20_2() {
        let str = include_str!("../day21.txt");
        assert_eq!(part1(str, 64), 3632);
    }
}
