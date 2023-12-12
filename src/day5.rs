pub fn day5() {
    let str = include_str!("../day5.txt");

    let total = part1(&str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i64 {
    let parts = str.split("\n\n").collect::<Vec<_>>();
    let seeds = parts[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut mat = Vec::new();
    for seed in seeds {
        let mut inner_vec = vec![-1; 8];
        inner_vec[0] = seed;
        mat.push(inner_vec);
    }
    for i in 1..8 {
        let lines = parts[i]
            .split('\n')
            .skip(1)
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.split(' ')
                    .filter(|l| !l.is_empty())
                    .map(|l| l.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        for element in &mut mat {
            element[i] = if let Some(row) = lines
                .iter()
                .find(|row| (row[1]..row[1] + row[2]).contains(&element[i - 1]))
            {
                row[0] + element[i - 1] - row[1]
            } else {
                element[i - 1]
            }
        }
    }
    mat.iter().filter_map(|v| Some(v[7])).min().unwrap()
}

fn part2(str: &str) -> i64 {
    0
}
