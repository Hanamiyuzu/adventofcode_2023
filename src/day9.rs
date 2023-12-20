pub fn day9() {
    let str = include_str!("../day9.txt");

    let total = part2(str);
    println!("total: {}", total);
}

fn part1(str: &str) -> i32 {
    let seqs = parse(str);
    seqs.iter().map(|seq| calc_next_sequence_num(seq)).sum()
}

fn part2(str: &str) -> i32 {
    let seqs = parse(str);
    seqs.iter().map(|seq| calc_prev_sequence_num(seq)).sum()
}

fn calc_next_sequence_num(seq: &[i32]) -> i32 {
    let diffs = seq.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();

    if diffs.iter().all(|&x| x == diffs[0]) {
        seq.last().unwrap() + diffs[0]
    } else {
        seq.last().unwrap() + calc_next_sequence_num(&diffs)
    }
}

fn calc_prev_sequence_num(seq: &[i32]) -> i32 {
    let diffs = seq.windows(2).map(|x| x[1] - x[0]).collect::<Vec<_>>();
    if diffs.iter().all(|&x| x == diffs[0]) {
        seq.first().unwrap() - diffs[0]
    } else {
        seq.first().unwrap() - calc_prev_sequence_num(&diffs)
    }
}

fn parse(str: &str) -> Vec<Vec<i32>> {
    str.lines()
        .map(|s| s.split(' ').map(|x| x.parse::<i32>().unwrap()).collect())
        .collect()
}
