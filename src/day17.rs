use std::collections::{BinaryHeap, HashSet};

pub fn day17() {
    let str = include_str!("../day17.txt");
    let timer = std::time::Instant::now();
    let total = part1(str);
    println!("total: {} in {:?}", total, timer.elapsed());
}

fn part1(str: &str) -> i32 {
    let map = str
        .lines()
        .map(|s| s.chars().map(|c| c as i32 - 48).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    shortest_path(&map)
}

#[derive(Eq, Debug, Clone, Hash)]
struct State {
    row: i32,
    col: i32,
    dir: (i32, i32),
    expend: i32,
    same_step: i32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.expend == other.expend
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.expend.cmp(&self.expend)
    }
}

fn shortest_path(map: &[Vec<i32>]) -> i32 {
    let (rows, cols) = (map.len() as i32, map[0].len() as i32);
    let mut queue = BinaryHeap::new();
    queue.push(State {
        row: 0,
        col: 0,
        dir: (0, 1),
        expend: 0,
        same_step: 0,
    });
    let mut visited = HashSet::with_capacity((rows * cols) as usize);
    while let Some(st) = queue.pop() {
        if st.row == rows - 1 && st.col == cols - 1 {
            return st.expend;
        }
        if !visited.insert((st.row, st.col, st.dir, st.same_step)) {
            continue;
        }
        if st.same_step < 3 {
            let (nrow, ncol) = (st.row + st.dir.0, st.col + st.dir.1);
            if nrow >= 0 && ncol >= 0 && nrow < rows && ncol < cols {
                queue.push(State {
                    row: nrow,
                    col: ncol,
                    dir: st.dir,
                    expend: st.expend + map[nrow as usize][ncol as usize],
                    same_step: st.same_step + 1,
                });
            }
        }
        for ndir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if ndir != st.dir && (-ndir.0, -ndir.1) != st.dir {
                let (nrow, ncol) = (st.row + ndir.0, st.col + ndir.1);
                if nrow >= 0 && ncol >= 0 && nrow < rows && ncol < cols {
                    queue.push(State {
                        row: nrow,
                        col: ncol,
                        dir: ndir,
                        expend: st.expend + map[nrow as usize][ncol as usize],
                        same_step: 1,
                    });
                }
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day17_1() {
        let str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(part1(str), 102);
    }

    #[test]
    fn test_day17_2() {
        let str = include_str!("../day17.txt");
        assert_eq!(part1(str), 959);
    }
}
