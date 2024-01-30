use std::collections::HashSet;

pub fn day23() {
    let str = include_str!("../day23.txt");
    let timer = std::time::Instant::now();
    let total = part1(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> i32 {
    let map = parse(str);
    dfs(&map)
}

fn dfs(map: &Vec<Vec<char>>) -> i32 {
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    let mut stack = vec![(0, (0, 1), false)];
    let mut visited = HashSet::new();
    let mut max_steps = 0;
    while let Some((steps, cur, rm)) = stack.pop() {
        if rm {
            visited.remove(&cur);
            continue;
        }
        if cur.0 < 0 || cur.0 >= rows || cur.1 < 0 || cur.1 >= cols {
            continue;
        }
        if !visited.insert(cur) {
            continue;
        }
        max_steps = max_steps.max(steps);
        stack.push((steps, cur, true)); // rm
        match map[cur.0 as usize][cur.1 as usize] {
            '.' => {
                if cur.0 == rows - 1 {
                    continue;
                }
                for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    stack.push((steps + 1, (cur.0 + dir.0, cur.1 + dir.1), false))
                }
            }
            '>' => stack.push((steps + 1, (cur.0, cur.1 + 1), false)),
            'v' => stack.push((steps + 1, (cur.0 + 1, cur.1), false)),
            '<' => stack.push((steps + 1, (cur.0, cur.1 - 1), false)),
            '^' => stack.push((steps + 1, (cur.0 - 1, cur.1), false)),
            _ => continue,
        }
    }
    max_steps
}

fn parse(str: &str) -> Vec<Vec<char>> {
    str.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day23_1() {
        let str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(part1(str), 94);
    }

    #[test]
    fn test_day23_2() {
        let str = include_str!("../day23.txt");
        assert_eq!(part1(str), 2030);
    }
}
