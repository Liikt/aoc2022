use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,

    UpLeft,
    UpRight,

    DownLeft,
    DownRight,
}

impl Direction {
    fn get_scan_area() -> [(isize, isize); 9] {
        [
            (-1,  1), ( 0,  1), ( 1,  1),
            (-1,  0), ( 0,  0), ( 1,  0),
            (-1, -1), ( 0, -1), ( 1, -1),
        ]
    }

    fn get_follow_area() -> [(isize, isize); 16] {
        [
            (-2,  2), (-1,  2), ( 0,  2), ( 1,  2), ( 2,  2),
            (-2,  1),                               ( 2,  1),
            (-2,  0),                               ( 2,  0),
            (-2, -1),                               ( 2, -1),
            (-2, -2), (-1, -2), ( 0, -2), ( 1, -2), ( 2, -2),
        ]
    }
}

impl From<(isize, isize)> for Direction {
    fn from(value: (isize, isize)) -> Self {
        match value {
            ( 0,  1) | ( 0,  2)            => Direction::Up,
            ( 0, -1) | ( 0, -2)            => Direction::Down,
            (-1,  0) | (-2,  0)            => Direction::Left,
            ( 1,  0) | ( 2,  0)            => Direction::Right,
            ( 1,  2) | ( 2,  1) | ( 2,  2) => Direction::UpRight,
            (-1,  2) | (-2,  1) | (-2,  2) => Direction::UpLeft,
            ( 1, -2) | ( 2, -1) | ( 2, -2) => Direction::DownRight,
            (-1, -2) | (-2, -1) | (-2, -2) => Direction::DownLeft,
            _ => panic!("Huh {:?}", value)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    direction: Direction,
    amount: usize
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn step(&self, dir: Direction) -> Self {
        let s = match dir {
            Direction::Up        => ( 0,  1),
            Direction::Down      => ( 0, -1),
            Direction::Left      => (-1,  0),
            Direction::Right     => ( 1,  0),
            Direction::UpRight   => ( 1,  1),
            Direction::UpLeft    => (-1,  1),
            Direction::DownRight => ( 1, -1),
            Direction::DownLeft  => (-1, -1),
        };

        self.adjust(s)
    }

    fn adjust(&self, dir: (isize, isize)) -> Self {
        Self {
            x: self.x + dir.0,
            y: self.y + dir.1,
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        let amount = value.split(" ").nth(1).unwrap().parse::<usize>().unwrap();
        let dir = match value.split(" ").nth(0).unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!()
        };
        Move { direction: dir, amount }
    }
}

fn process(input: String) -> Vec<Move> {
    input.split("\n").map(|x| Move::from(x)).collect()
}

pub fn solve1(input: String) -> usize {
    let moves = process(input);

    let mut head_pos = Position { x: 0, y: 0 };
    let mut tail_pos = Position { x: 0, y: 0 };

    let mut visited = HashSet::new();
    visited.insert(tail_pos);

    for m in moves {
        for _ in 0..m.amount {
            head_pos = head_pos.step(m.direction);
            if !Direction::get_scan_area().into_iter().any(|d| {
                tail_pos.adjust(d) == head_pos
            }) {
                let dir = Direction::get_follow_area().into_iter()
                    .filter(|d| tail_pos.adjust(*d) == head_pos).next()
                    .unwrap();
                tail_pos = tail_pos.step(dir.into());
                visited.insert(tail_pos);
            }
        }
    }

    visited.len()
}

fn print_state(knots: [Position; 10]) {
    for y in (-5..21-5).rev() {
        for x in -11..26-11 {
            let mut flag = true;
            'p: for (i, k) in knots.into_iter().enumerate() {
                let cur = Position {x, y};
                if cur == k {
                    if i == 0 {
                        print!("H");
                    } else {
                        print!("{}", i);
                    }
                    flag = false;
                    break 'p;
                }
            }
            if flag && x == 0 && y == 0 {
                print!("s");
            } else if flag {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn print_visited(visited: &HashSet<Position>) {
    for y in (-5..21-5).rev() {
        for x in -11..26-11 {
            let cur = Position { x, y };
            if visited.contains(&cur) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub fn solve2(input: String) -> usize {
    let moves = process(input);

    let mut knots = [Position { x: 0, y: 0 }; 10];

    let mut visited = HashSet::new();
    visited.insert(knots[9]);

    for m in moves {
        for _ in 0..m.amount {
            knots[0] = knots[0].step(m.direction);
            for knot in 1..knots.len() {
                if !Direction::get_scan_area().into_iter().any(|d| {
                    knots[knot].adjust(d) == knots[knot-1]
                }) {
                    let dir = Direction::get_follow_area().into_iter()
                        .filter(|d| knots[knot].adjust(*d) == knots[knot-1]).next()
                        .unwrap();
                    knots[knot] = knots[knot].step(dir.into());
                    if knot == knots.len() -1 {
                        visited.insert(knots[knot]);
                    }
                }
            }
        }
        if false {
            print_state(knots);
        }
    }

    if false {
        print_visited(&visited);
    }
    visited.len()
}