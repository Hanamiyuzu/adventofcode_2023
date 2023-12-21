pub fn day2() {
    let strs = include_str!("../day2.txt").lines().collect::<Vec<_>>();

    let mut total = 0;
    for str in &strs {
        total += part2(str);
    }
    println!("total: {}", total);
}

const COLOR_CUBES: (u32, u32, u32) = (12, 13, 14);

fn part1(str: &str) -> i32 {
    if let Some((first, last)) = str.split_once(':') {
        let id = first[5..].parse::<i32>().unwrap();
        let cubes_sets = last
            .split("; ")
            .map(|s| {
                s.trim()
                    .split(',')
                    .map(|s| s.trim().split_once(' ').unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        for num_colors in cubes_sets {
            let (mut red, mut green, mut blue) = (0, 0, 0);
            for (num, color) in num_colors {
                match color {
                    "red" => red += num.parse::<u32>().unwrap(),
                    "green" => green += num.parse::<u32>().unwrap(),
                    "blue" => blue += num.parse::<u32>().unwrap(),
                    _ => (),
                }
            }
            if red > COLOR_CUBES.0 || green > COLOR_CUBES.1 || blue > COLOR_CUBES.2 {
                return 0;
            }
        }
        return id;
    }
    0
}

fn part2(str: &str) -> i32 {
    if let Some((_, last)) = str.split_once(':') {
        let num_colors = last
            .split(&[',', ';'])
            .map(|s| s.trim().split_once(' ').unwrap())
            .collect::<Vec<_>>();

        let (mut red, mut green, mut blue) = (1, 1, 1);
        for (num, color) in num_colors {
            let num = num.parse::<i32>().unwrap();
            match color {
                "red" if red < num => {
                    red = num;
                }
                "green" if green < num => {
                    green = num;
                }
                "blue" if blue < num => {
                    blue = num;
                }
                _ => (),
            }
        }
        return red * green * blue;
    }
    0
}
