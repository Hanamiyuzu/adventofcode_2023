use std::collections::HashSet;

pub fn day16() {
    let str = include_str!("../day16.txt");
    let timer = std::time::Instant::now();
    let total = part2(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> i32 {
    let map = parse(str);
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut cache = HashSet::with_capacity(4 * map.len() * map[0].len());

    refract(&map, (0, 0, DIR::RIGHT), &mut visited, &mut cache);

    visited.iter().flatten().filter(|&&x| x).count() as i32
}

fn part2(str: &str) -> i32 {
    //todo!("Code will stack overflow in debug. Replace it with dp when I have time.");

    let map = parse(str);

    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    assert!(rows > 0 && cols > 0);

    *[
        (0..cols)
            .map(|col| {
                let mut cache = HashSet::with_capacity(4 * map.len() * map[0].len());
                let mut visited = vec![vec![false; map[0].len()]; map.len()];
                refract(&map, (0, col as i32, DIR::BOTTOM), &mut visited, &mut cache);
                visited.iter().flatten().filter(|&&x| x).count() as i32
            })
            .max()
            .unwrap(),
        (0..cols)
            .map(|col| {
                let mut cache = HashSet::with_capacity(4 * map.len() * map[0].len());
                let mut visited = vec![vec![false; map[0].len()]; map.len()];
                refract(
                    &map,
                    (rows - 1, col as i32, DIR::TOP),
                    &mut visited,
                    &mut cache,
                );
                visited.iter().flatten().filter(|&&x| x).count() as i32
            })
            .max()
            .unwrap(),
        (0..rows)
            .map(|row| {
                let mut cache = HashSet::with_capacity(4 * map.len() * map[0].len());
                let mut visited = vec![vec![false; map[0].len()]; map.len()];
                refract(&map, (row as i32, 0, DIR::RIGHT), &mut visited, &mut cache);
                visited.iter().flatten().filter(|&&x| x).count() as i32
            })
            .max()
            .unwrap(),
        (0..rows)
            .map(|row| {
                let mut cache = HashSet::with_capacity(4 * map.len() * map[0].len());
                let mut visited = vec![vec![false; map[0].len()]; map.len()];
                refract(
                    &map,
                    (row as i32, cols - 1, DIR::LEFT),
                    &mut visited,
                    &mut cache,
                );
                visited.iter().flatten().filter(|&&x| x).count() as i32
            })
            .max()
            .unwrap(),
    ]
    .iter()
    .max()
    .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DIR {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT,
}

fn refract(
    map: &[Vec<char>],
    (row, col, dir): (i32, i32, DIR),
    visited: &mut Vec<Vec<bool>>,
    cache: &mut HashSet<(i32, i32, DIR)>,
) {
    if row < 0 || row >= map.len() as i32 || col < 0 || col >= map[0].len() as i32 {
        return;
    }
    if cache.contains(&(row, col, dir)) {
        return;
    }
    cache.insert((row, col, dir));
    visited[row as usize][col as usize] = true;
    match map[row as usize][col as usize] {
        '.' => {
            refract(map, to_next(row, col, dir), visited, cache);
        }
        '/' => {
            let dir = match dir {
                DIR::TOP => DIR::RIGHT,
                DIR::RIGHT => DIR::TOP,
                DIR::BOTTOM => DIR::LEFT,
                DIR::LEFT => DIR::BOTTOM,
            };
            refract(map, to_next(row, col, dir), visited, cache);
        }
        '\\' => {
            let dir = match dir {
                DIR::TOP => DIR::LEFT,
                DIR::RIGHT => DIR::BOTTOM,
                DIR::BOTTOM => DIR::RIGHT,
                DIR::LEFT => DIR::TOP,
            };
            refract(map, to_next(row, col, dir), visited, cache);
        }
        '|' => match dir {
            DIR::LEFT | DIR::RIGHT => {
                refract(map, to_next(row, col, DIR::TOP), visited, cache);
                refract(map, to_next(row, col, DIR::BOTTOM), visited, cache);
            }
            _ => refract(map, to_next(row, col, dir), visited, cache),
        },
        '-' => match dir {
            DIR::TOP | DIR::BOTTOM => {
                refract(map, to_next(row, col, DIR::LEFT), visited, cache);
                refract(map, to_next(row, col, DIR::RIGHT), visited, cache);
            }
            _ => refract(map, to_next(row, col, dir), visited, cache),
        },
        _ => panic!(),
    }
}

fn to_next(row: i32, col: i32, dir: DIR) -> (i32, i32, DIR) {
    match dir {
        DIR::TOP => (row - 1, col, dir),
        DIR::RIGHT => (row, col + 1, dir),
        DIR::BOTTOM => (row + 1, col, dir),
        DIR::LEFT => (row, col - 1, dir),
    }
}

fn parse(str: &str) -> Vec<Vec<char>> {
    str.lines().map(|s| s.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day16_1() {
        let str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(part1(str), 46);
        assert_eq!(part2(str), 51);
    }

    #[test]
    fn test_day16_2() {
        let str = include_str!("../day16.txt");
        assert_eq!(part1(str), 8389);
        assert_eq!(part2(str), 8564);
    }
}
