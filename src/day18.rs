pub fn day18() {
    let str = include_str!("../day18.txt");
    let timer = std::time::Instant::now();
    let total = part2(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> i32 {
    let polygon = walk(&parse(str));

    let mut area = 0;
    for pair in polygon.windows(2) {
        area += pair[0].0 * pair[1].1;
        area -= pair[0].1 * pair[1].0;
    }
    let area = area.abs() / 2;
    area - (polygon.len() as i32 / 2) + 1 + polygon.len() as i32 - 1
}

fn part2(str: &str) -> i64 {
    let add_tuples = |a: &(i64, i64), b: &(i64, i64)| (a.0 + b.0, a.1 + b.1);
    let mul_tuple_int = |a: &(i64, i64), b: i64| (a.0 * b, a.1 * b);

    let mut count = 1;
    let mut area = 0;
    let mut dots = [(0, 0); 2];
    str.lines().for_each(|s| {
        let s = s.rsplit_once(' ').unwrap().1;
        let steps = i64::from_str_radix(&s[2..7], 16).unwrap();
        let dir = s.chars().nth(7).unwrap() as usize - 48;
        {
            dots[1] = add_tuples(
                &dots[0],
                &mul_tuple_int(&[(0, 1), (1, 0), (0, -1), (-1, 0)][dir], steps),
            );
            area += dots[0].0 * dots[1].1;
            area -= dots[0].1 * dots[1].0;
            dots[0] = dots[1];
        }
        count += steps;
    });
    let area = area.abs() / 2;
    area - (count / 2) + 1 + count - 1
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

fn parse(str: &str) -> Vec<(Dir, i32, &str)> {
    str.lines()
        .map(|s| {
            (
                match s.chars().nth(0).unwrap() {
                    'U' => Dir::Up,
                    'D' => Dir::Down,
                    'L' => Dir::Left,
                    'R' => Dir::Right,
                    _ => panic!(),
                },
                s[2..]
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
                s.rsplit_once(' ').unwrap().1,
            )
        })
        .collect::<Vec<_>>()
}

fn walk(rules: &[(Dir, i32, &str)]) -> Vec<(i32, i32)> {
    let add_tuples = |a: &(i32, i32), b: &(i32, i32)| (a.0 + b.0, a.1 + b.1);
    let mut res = vec![(0, 0)];
    rules.iter().for_each(|(dir, steps, _)| {
        for _ in 0..*steps {
            res.push(add_tuples(
                res.last().unwrap(),
                &[(0, 1), (1, 0), (0, -1), (-1, 0)][*dir as usize],
            ));
        }
    });
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day18_1() {
        let str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(part1(str), 62);
        assert_eq!(part2(str), 952408144115);
    }

    #[test]
    fn test_day18_2() {
        let str = include_str!("../day18.txt");
        assert_eq!(part1(str), 35244);
        assert_eq!(part2(str), 85070763635666);
    }
}
