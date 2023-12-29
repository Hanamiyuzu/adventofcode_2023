use std::collections::HashMap;

pub fn day14() {
    let str = include_str!("../day14.txt");
    let timer = std::time::Instant::now();
    let total = part2(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> usize {
    let map = parse(str);

    let mut total_load = 0;
    let mut loads = vec![map.len(); map[0].len()];
    map.iter().enumerate().for_each(|(row, s)| {
        s.iter().enumerate().for_each(|(col, c)| match c {
            'O' => {
                total_load += loads[col];
                loads[col] -= 1;
            }
            '#' => {
                loads[col] = map.len() - row - 1;
            }
            _ => (),
        });
    });
    total_load
}

fn part2(str: &str) -> usize {
    let mut map = parse(str);

    let mut hashmap = HashMap::new();
    (1..).find(|&n| {
        once_cycle(&mut map);
        if let Some(x) = hashmap.insert(map.clone(), n) {
            if (1000000000 - n) % (n - x) == 0 {
                return true;
            }
        }
        false
    });

    (0..map.len())
        .map(|row| {
            map[row]
                .iter()
                .filter(|&&c| c == 'O')
                .map(|_| map.len() - row)
                .sum::<usize>()
        })
        .sum()
}

fn parse(str: &str) -> Vec<Vec<char>> {
    str.split('\n').map(|s| s.chars().collect()).collect()
}

fn north_roll(map: &mut Vec<Vec<char>>) {
    let mut next_place = vec![0; map[0].len()];
    (0..map.len()).for_each(|row| {
        (0..map[row].len()).for_each(|col| match map[row][col] {
            'O' => {
                if row != next_place[col] {
                    map[next_place[col]][col] = 'O';
                    map[row][col] = '.';
                }
                next_place[col] += 1;
            }
            '#' => {
                next_place[col] = row + 1;
            }
            _ => (),
        })
    })
}

// rows == cols
fn inplace_right_rotate(map: &mut Vec<Vec<char>>) {
    assert_eq!(map.len(), map[0].len());
    for i in 0..map.len() {
        for j in i + 1..map.len() {
            let temp = map[i][j];
            map[i][j] = map[j][i];
            map[j][i] = temp;
        }
    }
    for row in map {
        row.reverse();
    }
}

fn once_cycle(map: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        north_roll(map);
        inplace_right_rotate(map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14_1() {
        let str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(part1(str), 136);
        assert_eq!(part2(str), 64);
    }

    #[test]
    fn test_day14_2() {
        let str = include_str!("../day14.txt");
        assert_eq!(part1(str), 110274);
        assert_eq!(part2(str), 90982);
    }
}
