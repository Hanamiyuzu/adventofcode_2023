pub fn day5() {
    let str = include_str!("../day5.txt");

    let total = part2(&str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i64 {
    let (parts, seeds) = parse(str);

    let mut mat = Vec::new();
    for seed in seeds {
        let mut inner_vec = vec![-1; 8];
        inner_vec[0] = seed;
        mat.push(inner_vec);
    }
    calc(&parts, mat)
}

// part2 may require nearly 10GB of memory usage in the release environment.
const SLICE_BATCH: i64 = 104857600;

fn part2(str: &str) -> i64 {
    let (parts, seeds) = parse(str);

    let mut res = Vec::new();
    for pair in seeds.chunks_exact(2) {
        let time = std::time::Instant::now();
        (0..pair[1])
            .step_by(SLICE_BATCH as usize)
            .for_each(|start| {
                let end = std::cmp::min(pair[1], start + SLICE_BATCH);
                let mat = (start..end)
                    .map(|i| {
                        let mut inner_vec = vec![-1; 8];
                        inner_vec[0] = pair[0] + i;
                        inner_vec
                    })
                    .collect::<Vec<_>>();
                res.push(calc(&parts, mat));
            });
        println!("elapsed: {:?}", time.elapsed());
    }
    *res.iter().min().unwrap()
}

fn parse(str: &str) -> (Vec<&str>, Vec<i64>) {
    let parts = str.split("\n\n").collect::<Vec<_>>();
    let seeds = parts[0]
        .strip_prefix("seeds: ")
        .unwrap()
        .split(' ')
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    (parts, seeds)
}

fn calc(parts: &Vec<&str>, mut mat: Vec<Vec<i64>>) -> i64 {
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
