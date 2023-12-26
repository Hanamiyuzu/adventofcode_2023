pub fn day11() {
    let str = include_str!("../day11.txt");

    let total = part2(str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i32 {
    let stars = parse(str, 2);
    calc(&stars)
}

fn part2(str: &str) -> i64 {
    let stars = parse(str, 1000000);
    calc(&stars)
}

fn parse(str: &str, factor: i32) -> Vec<(i32, i32)> {
    let star_map = str
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let row_all_empty = |row: usize| star_map[row].iter().all(|c| c == &'.');
    let col_all_empty = |col: usize| star_map.iter().all(|r| r[col] == '.');

    let mut stars = Vec::new();
    let mut empty_cols = Vec::new();
    let mut row = 0;
    for (i, rows) in star_map.iter().enumerate() {
        let mut col = 0;
        let mut skip_row_empty = false;
        for (j, c) in rows.iter().enumerate() {
            if c == &'#' {
                stars.push((row, col));
                skip_row_empty = true;
            } else {
                if empty_cols.contains(&j) {
                    col += factor - 1;
                } else if col_all_empty(j) {
                    col += factor - 1;
                    empty_cols.push(j);
                }
            }
            col += 1;
        }
        if !skip_row_empty && row_all_empty(i) {
            row += factor - 1;
        }
        row += 1;
    }
    stars
}

fn calc<T>(stars: &Vec<(i32, i32)>) -> T
where
    T: From<i32> + std::iter::Sum,
{
    stars
        .iter()
        .enumerate()
        .map(|(i, s1)| {
            stars
                .iter()
                .skip(i + 1)
                .map(|s2| T::from((s1.0 - s2.0).abs() + (s1.1 - s2.1).abs()))
                .sum()
        })
        .sum()
}
