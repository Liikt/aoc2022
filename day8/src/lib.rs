#[derive(Debug, Default, Clone, Copy)]
struct Tree {
    north: i8,
    east:  i8,
    south: i8,
    west:  i8,

    height: u8
}

#[derive(Debug, Default, Clone)]
struct Forest {
    trees:  Vec<Vec<Tree>>,
    height: usize,
    width:  usize
}

impl Forest {
    fn populate(&mut self) {
        let copy = self.clone();

        // visible from north
        let mut max = vec![-1; self.width];
        for (y, row) in copy.trees.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col.height as i8 > max[x] {
                    max[x] = col.height as i8;
                }
                self.trees[y][x].north = max[x];
            }
        }

        // visible from south
        let mut max = vec![-1; self.width];
        for (y, row) in copy.trees.iter().rev().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if col.height as i8 > max[x] {
                    max[x] = col.height as i8;
                }
                self.trees[self.height-1-y][x].south = max[x];
            }
        }

        // visible from east
        for (y, row) in copy.trees.iter().enumerate() {
            let mut max = -1;
            for (x, col) in row.iter().enumerate() {
                if col.height as i8 > max {
                    max = col.height as i8;
                }
                self.trees[y][x].east = max;
            }
        }

        // visible from west
        for (y, row) in copy.trees.iter().enumerate() {
            let mut max = -1;
            for (x, col) in row.iter().rev().enumerate() {
                if col.height as i8 > max {
                    max = col.height as i8;
                }
                self.trees[y][self.height-1-x].west = max;
            }
        }
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        if row == 0 || row == self.height-1 || col == 0 || col == self.width-1 {
            return true;
        }

        let cur   = self.trees[row][col];
        let north = self.trees[row-1][col].north < cur.height as i8;
        let south = self.trees[row+1][col].south < cur.height as i8;
        let east  = self.trees[row][col-1].east  < cur.height as i8;
        let west  = self.trees[row][col+1].west  < cur.height as i8;

        return north || south || east || west;
    }

    fn scenic_score(&self, row: usize, col: usize) -> usize {
        let mut total = 1;
        if row == 0 || row == self.height-1 || col == 0 || col == self.width-1 {
            return 0;
        }

        let cur = self.trees[row][col];

        // north
        let mut r = row-1;
        let mut tmp = 1;
        loop {
            if self.trees[r][col].height >= cur.height || r == 0 {
                break;
            }
            if self.trees[r][col].height <= cur.height {
                tmp += 1;
            }
            r -= 1;
        }
        total *= tmp;

        // south
        let mut r = row+1;
        let mut tmp = 1;
        loop {
            if self.trees[r][col].height >= cur.height || r == self.height-1 {
                break;
            }
            if self.trees[r][col].height <= cur.height {
                tmp += 1;
            }
            r += 1;
        }
        total *= tmp;

        // east
        let mut c = col-1;
        let mut tmp = 1;
        loop {
            if self.trees[row][c].height >= cur.height || c == 0 {
                break;
            }
            if self.trees[row][c].height <= cur.height {
                tmp += 1;
            }
            c -= 1;
        }
        total *= tmp;

        // west
        let mut c = col+1;
        let mut tmp = 1;
        loop {
            if self.trees[row][c].height >= cur.height || c == self.width-1 {
                break;
            }
            if self.trees[row][c].height <= cur.height {
                tmp += 1;
            }
            c += 1;
        }
        total *= tmp;

        total
    }
}

fn process(input: String) -> Forest {
    let mut ret = Forest {
        trees: input.split("\n").map(|x| x.as_bytes().into_iter().map(|&y| {
            Tree { height: y - 0x30, north: -1, east: -1, south: -1, west: -1 }
        }).collect()).collect(),
        ..Default::default()
    };
    ret.height = ret.trees.len();
    ret.width = ret.trees[0].len();
    ret.populate();
    ret
}

pub fn solve1(input: String) -> usize {
    let inp= process(input);
    let mut total = 0;

    for row in 0..inp.height {
        for col in 0..inp.width {
            if inp.is_visible(row, col) { total += 1; }
        }
    }

    total
}

pub fn solve2(input: String) -> usize {
    let inp= process(input);
    let mut max = 0;

    for row in 0..inp.height {
        for col in 0..inp.width {
            max = inp.scenic_score(row, col).max(max);
        }
    }

    max
}