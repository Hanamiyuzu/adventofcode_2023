pub fn day10() {
    let str = include_str!("../day10.txt");

    let total = part1(str);
    println!("total: {}", total);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    pos: (i32, i32),
    value: char,
    directions: Vec<(i32, i32)>,
    is_start: bool,
}

impl Node {
    fn new() -> Node {
        Node {
            pos: (0, 0),
            value: ' ',
            directions: vec![],
            is_start: false,
        }
    }

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
    start: Node,
}

impl Maze {
    pub fn walk_loop(&self) -> Vec<&Node> {
        let mut next_pos = *[
            (self.start.pos.0, self.start.pos.1 - 1),
            (self.start.pos.0, self.start.pos.1 + 1),
            (self.start.pos.0 - 1, self.start.pos.1),
            (self.start.pos.0 + 1, self.start.pos.1),
        ]
        .iter()
        .find(|(i, j)| {
            self.mat[*i as usize][*j as usize]
                .directions
                .contains(&self.start.pos)
        })
        .unwrap();
        let mut prev_pos = self.start.pos;
        let mut cur_node = &self.mat[next_pos.0 as usize][next_pos.1 as usize];
        let mut visited = vec![cur_node];
        while cur_node != &self.start {
            next_pos = cur_node.next_position(prev_pos);
            prev_pos = cur_node.pos;
            cur_node = &self.mat[next_pos.0 as usize][next_pos.1 as usize];
            visited.push(cur_node);
        }
        visited
    }
}

fn part1(str: &str) -> i32 {
    let maze = parse(str);
    let path = maze.walk_loop();
    path.len() as i32 / 2
}

fn parse(str: &str) -> Maze {
    let mut res = Maze {
        mat: vec![],
        start: Node::new(),
    };
    str.lines().enumerate().for_each(|(i, s)| {
        let mut nodes = vec![];
        s.chars().enumerate().for_each(|(j, c)| {
            let (row, col) = (i as i32, j as i32);
            let node = match c {
                '|' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![(row - 1, col), (row + 1, col)],
                    is_start: false,
                },
                '-' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![(row, col - 1), (row, col + 1)],
                    is_start: false,
                },
                'L' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![(row - 1, col), (row, col + 1)],
                    is_start: false,
                },
                'J' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![(row, col - 1), (row - 1, col)],
                    is_start: false,
                },
                '7' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![(row, col - 1), (row + 1, col)],
                    is_start: false,
                },
                'F' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![(row, col + 1), (row + 1, col)],
                    is_start: false,
                },
                '.' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![],
                    is_start: false,
                },
                'S' => Node {
                    pos: (row, col),
                    value: c,
                    directions: vec![],
                    is_start: true,
                },
                _ => panic!(),
            };
            if node.is_start {
                res.start = node.clone();
            }
            nodes.push(node);
        });
        res.mat.push(nodes);
    });
    res
}
