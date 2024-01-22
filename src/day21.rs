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
                if row >= 0
                    && row < map.len() as i32
                    && col >= 0
                    && col < map[0].len() as i32
                    && map[row as usize][col as usize] != '#'
                {
                    q1.insert(((row, col), step - 1));
                }
            }
        }
    }
    q1.len() as i32
}

fn parse(str: &str) -> ((i32, i32), Vec<Vec<char>>) {
    let mut start = (0, 0);
    let map = str
        .lines()
        .enumerate()
        .map(|(row, s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start = (row as i32, col as i32);
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
