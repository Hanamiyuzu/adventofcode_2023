pub fn day6() {
    let str = include_str!("../day6.txt");

    let total = part2(&str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i64 {
    let arr = str
        .lines()
        .map(|s| {
            s.split_once(": ")
                .unwrap()
                .1
                .split(' ')
                .filter(|l| !l.is_empty())
                .map(|l| l.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    arr[0]
        .iter()
        .zip(arr[1].iter())
        .map(|(time, distance)| calc(*time, *distance))
        .product()
}

fn part2(str: &str) -> i64 {
    let arr = str
        .lines()
        .map(|s| {
            s.split_once(": ")
                .unwrap()
                .1
                .replace(" ", "")
                .parse::<i64>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    let (time, distance) = (arr[0], arr[1]);
    calc(time, distance)
}

fn calc(time: i64, distance: i64) -> i64 {
    // x^2 - time * x + move = 0
    let (a, b, c) = (1.0, -time as f64, distance as f64);
    let x1 = (-b - (b.powi(2) - 4.0 * a * c).sqrt()) / (2.0 * a);
    let min_hold = (x1 + 1.0) as i64;
    time - min_hold - min_hold + 1
}
