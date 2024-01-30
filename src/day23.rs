use std::collections::HashSet;

pub fn day23() {
    let str = include_str!("../day23.txt");
    let timer = std::time::Instant::now();
    let total = part1(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> i32 {
    let map = parse(str);
    let cur = (0, map[0].iter().position(|&x| x == '.').unwrap() as i32);
    let mut visited = HashSet::new();
    dfs(&mut visited, &map, cur)
}

fn dfs(visited: &mut HashSet<(i32, i32)>, map: &Vec<Vec<char>>, cur: (i32, i32)) -> i32 {
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    if cur.0 < 0 || cur.0 >= rows || cur.1 < 0 || cur.1 >= cols {
        return 0;
    }
    if !visited.insert(cur) {
        return 0;
    }
    let steps = match map[cur.0 as usize][cur.1 as usize] {
        '#' => 0,
        '.' => {
            if cur.0 == rows - 1 {
                return 1;
            }
            [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .map(|&dir| dfs(visited, map, (cur.0 + dir.0, cur.1 + dir.1)) + 1)
                .max()
                .unwrap()
        }
        '>' => dfs(visited, map, (cur.0, cur.1 + 1)) + 1,
        'v' => dfs(visited, map, (cur.0 + 1, cur.1)) + 1,
        '<' => dfs(visited, map, (cur.0, cur.1 - 1)) + 1,
        '^' => dfs(visited, map, (cur.0 - 1, cur.1)) + 1,
        _ => unreachable!(),
    };
    visited.remove(&cur);
    steps
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
