pub fn day10() {
    let str = include_str!("../day10.txt");

    let total = part2(str);
    println!("total: {}", total);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: (i32, i32),
    directions: Vec<(i32, i32)>,
    //value: char,
}

impl Node {
    fn next_position(&self, from: (i32, i32)) -> (i32, i32) {
        if from == self.directions[0] {
            self.directions[1]
        } else {
            self.directions[0]
        }
    }
}

#[derive(Debug, Clone)]
struct Maze {
    mat: Vec<Vec<Node>>,
    start: (i32, i32),
}

impl Maze {
    pub fn walk(&self) -> Vec<(i32, i32)> {
        let mut cur = *[
            (self.start.0, self.start.1 - 1),
            (self.start.0, self.start.1 + 1),
            (self.start.0 - 1, self.start.1),
            (self.start.0 + 1, self.start.1),
        ]
        .iter()
        .find(|(i, j)| {
            if *i < 0 || *j < 0 || *i >= self.mat.len() as i32 || *j >= self.mat[0].len() as i32 {
                return false;
            }
            self.mat[*i as usize][*j as usize]
                .directions
                .contains(&self.start)
        })
        .unwrap();
        let mut prev = self.start;
        let mut visited = vec![cur];
        while cur != self.start {
            let node = &self.mat[cur.0 as usize][cur.1 as usize];
            let tmp = cur;
            cur = node.next_position(prev);
            prev = tmp;
            visited.push(cur);
        }
        visited
    }
}

fn part1(str: &str) -> i32 {
    let maze = parse(str);
    let polygon = maze.walk();
    polygon.len() as i32 / 2
}

// Ray Casting Algorithm or Shoelace_formula
fn part2(str: &str) -> i32 {
    let maze = parse(str);
    let mut polygon = maze.walk();
    // close the polygon
    polygon.push(*polygon.first().unwrap());

    // shoelace formula
    let mut area = 0;
    for pair in polygon.windows(2) {
        area += pair[0].0 * pair[1].1;
        area -= pair[0].1 * pair[1].0;
    }
    let area = area.abs() / 2;

    // Pick's theorem
    // Area = I + (B / 2) - 1
    area - (polygon.len() as i32 / 2) + 1
}

fn parse(str: &str) -> Maze {
    let mut res = Maze {
        mat: vec![],
        start: (0, 0),
    };
    str.lines().enumerate().for_each(|(i, s)| {
        let nodes = s
            .chars()
            .enumerate()
            .map(|(j, c)| {
                let (row, col) = (i as i32, j as i32);
                let directions = match c {
                    '|' => vec![(row - 1, col), (row + 1, col)],
                    '-' => vec![(row, col - 1), (row, col + 1)],
                    'L' => vec![(row - 1, col), (row, col + 1)],
                    'J' => vec![(row, col - 1), (row - 1, col)],
                    '7' => vec![(row, col - 1), (row + 1, col)],
                    'F' => vec![(row, col + 1), (row + 1, col)],
                    '.' | 'S' => vec![],
                    _ => panic!(),
                };
                if c == 'S' {
                    res.start = (row, col);
                }
                Node {
                    pos: (row, col),
                    directions,
                    //value: c,
                }
            })
            .collect();
        res.mat.push(nodes);
    });
    res
}
