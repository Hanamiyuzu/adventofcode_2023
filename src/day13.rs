pub fn day13() {
    let str = include_str!("../day13.txt");
    let timer = std::time::Instant::now();
    let total = part2(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> usize {
    parse(str)
        .iter()
        .map(|pattern| {
            find_vertical(pattern).map(|(x, _)| x + 1).unwrap_or(0)
                + find_horizontal(pattern)
                    .map(|(x, _)| (x + 1) * 100)
                    .unwrap_or(0)
        })
        .sum()
}

fn part2(str: &str) -> usize {
    parse(str)
        .iter()
        .map(|pattern| {
            find_smudge_vertical(pattern)
                .map(|(x, _)| x + 1)
                .unwrap_or(0)
                + find_smudge_horizontal(pattern)
                    .map(|(x, _)| (x + 1) * 100)
                    .unwrap_or(0)
        })
        .sum()
}

fn parse(str: &str) -> Vec<Vec<&str>> {
    str.split("\n\n")
        .map(|s| s.split('\n').collect::<Vec<_>>())
        .collect()
}

fn find_horizontal(pattern: &[&str]) -> Option<(usize, usize)> {
    let row_equals = |i: usize, j: usize| pattern[i] == pattern[j];
    let perfect_reflection = |i: usize, j: usize| {
        (0..i)
            .rev()
            .zip(j + 1..pattern.len())
            .all(|(i, j)| row_equals(i, j))
    };
    (0..pattern.len() - 1)
        .find(|&row| row_equals(row, row + 1) && perfect_reflection(row, row + 1))
        .map(|x| (x, x + 1))
}

fn find_vertical(pattern: &[&str]) -> Option<(usize, usize)> {
    let col_equals = |i: usize, j: usize| {
        (0..pattern.len()).all(|row| pattern[row].as_bytes()[i] == pattern[row].as_bytes()[j])
    };
    let perfect_reflection = |i: usize, j: usize| {
        (0..i)
            .rev()
            .zip(j + 1..pattern[0].len())
            .all(|(i, j)| col_equals(i, j))
    };
    (0..pattern[0].len() - 1)
        .find(|&col| col_equals(col, col + 1) && perfect_reflection(col, col + 1))
        .map(|x| (x, x + 1))
}

fn find_smudge_horizontal(pattern: &[&str]) -> Option<(usize, usize)> {
    let row_equals = |i: usize, j: usize, smudged: &mut bool| {
        for col in 0..pattern[0].len() {
            if pattern[i].as_bytes()[col] != pattern[j].as_bytes()[col] {
                if !*smudged {
                    *smudged = true;
                } else {
                    return false;
                }
            }
        }
        true
    };
    let smudged_reflection = |i: usize, j: usize, smudged: &mut bool| {
        (0..i)
            .rev()
            .zip(j + 1..pattern.len())
            .all(|(i, j)| row_equals(i, j, smudged))
    };
    (0..pattern.len() - 1)
        .find(|&row| {
            let mut smudged = false;
            row_equals(row, row + 1, &mut smudged)
                && smudged_reflection(row, row + 1, &mut smudged)
                && smudged // must be smudged
        })
        .map(|x| (x, x + 1))
}

fn find_smudge_vertical(pattern: &[&str]) -> Option<(usize, usize)> {
    let col_equals = |i: usize, j: usize, smudged: &mut bool| {
        for row in 0..pattern.len() {
            if pattern[row].as_bytes()[i] != pattern[row].as_bytes()[j] {
                if !*smudged {
                    *smudged = true;
                } else {
                    return false;
                }
            }
        }
        true
    };
    let smudged_reflection = |i: usize, j: usize, smudged: &mut bool| {
        (0..i)
            .rev()
            .zip(j + 1..pattern[0].len())
            .all(|(i, j)| col_equals(i, j, smudged))
    };
    (0..pattern[0].len() - 1)
        .find(|&col| {
            let mut smudged = false;
            col_equals(col, col + 1, &mut smudged)
                && smudged_reflection(col, col + 1, &mut smudged)
                && smudged // must be smudged
        })
        .map(|x| (x, x + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day13_1() {
        let str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(part1(str), 405);
        assert_eq!(part2(str), 400);
    }

    #[test]
    fn test_day13_2() {
        let str = include_str!("../day13.txt");

        assert_eq!(part1(str), 32371);
        assert_eq!(part2(str), 37416);
    }
}
