use std::collections::BTreeSet;

pub fn day3() {
    let strs = include_str!("../day3.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut total = part2(&strs);
    println!("total: {}", total);
}

fn part1(map: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;
    let mut already_done = BTreeSet::new();

    for (i, line) in map.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == '.' || c.is_digit(10) {
                continue;
            }
            let res = check_eight_conntected(&map, i as i32, j as i32);
            for (m, n) in res {
                let (row, col, num) = get_whole_num(map, m, n);
                if !already_done.contains(&(row, col)) {
                    already_done.insert((row, col));
                    total += num;
                }
            }
        }
    }
    total
}

fn part2(map: &Vec<Vec<char>>) -> i32 {
    let mut total = 0;

    for (i, line) in map.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == '.' || c.is_digit(10) {
                continue;
            }
            let mut nums = BTreeSet::new();
            let res = check_eight_conntected(&map, i as i32, j as i32);
            for (m, n) in res {
                let (.., num) = get_whole_num(map, m, n);
                nums.insert(num);
            }
            if nums.len() == 2 {
                total += nums.first().unwrap() * nums.last().unwrap();
            }
        }
    }
    total
}

fn check_eight_conntected(map: &Vec<Vec<char>>, row: i32, col: i32) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    let eight_conntected = [
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ];
    for (i, j) in eight_conntected {
        if i < 0 || j < 0 || i >= map.len() as i32 || j >= map[0].len() as i32 {
            continue;
        }
        let c = map[i as usize][j as usize];
        if c.is_digit(10) {
            res.push((i as usize, j as usize));
        }
    }
    res
}

fn get_whole_num(map: &Vec<Vec<char>>, row: usize, col: usize) -> (usize, usize, i32) {
    let mut col = col;
    while col > 0 {
        let c = map[row][col - 1];
        if c.is_digit(10) {
            col -= 1;
        } else {
            break;
        }
    }
    (
        row,
        col,
        map[row][col..]
            .iter()
            .take_while(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<i32>()
            .unwrap(),
    )
}
