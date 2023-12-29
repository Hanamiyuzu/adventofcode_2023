pub fn day14() {
    let str = include_str!("../day14.txt");
    let timer = std::time::Instant::now();
    let total = part1(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> usize {
    let map = parse(str);

    let mut total_load = 0;
    let mut loads = vec![map.len(); map[0].len()];
    map.iter().enumerate().for_each(|(row, s)| {
        s.chars().enumerate().for_each(|(col, c)| match c {
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

fn parse(str: &str) -> Vec<&str> {
    str.split('\n').collect()
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
    }

    #[test]
    fn test_day14_2() {
        let str = include_str!("../day14.txt");
        assert_eq!(part1(str), 110274);
    }
}
