use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Pos {
    pub x: isize,
    pub y: isize,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'P{{{},{}}}'", self.x, self.y)
    }
}

impl Pos {
    pub fn from<T: Into<isize>>(x: T, y: T) -> Pos {
        Pos {
            x: isize::try_from(x).unwrap(),
            y: isize::try_from(y).unwrap(),
        }
    }
    pub fn is_line(p1: Pos, p2: Pos, p3: Pos) -> bool {
        let dxc = p1.x - p2.x;
        let dyc = p1.y - p2.y;

        let dxl = p3.x - p2.x;
        let dyl = p3.y - p2.y;

        let cross = dxc * dyl - dyc * dxl;
        cross == 0
    }
}

pub struct GridIterator {
    curr: Pos,
    width: isize,
    height: isize,
}

impl Iterator for GridIterator {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.curr.x, self.curr.y);
        if x + 1 < self.width {
            self.curr = Pos { x: x+1, y };
            return Some(self.curr);
        } else if y + 1 < self.height {
            self.curr = Pos { x: 0, y: y+1 };
            return Some(self.curr);
        }
        None
    }
}

pub trait Block {
    type BT;
    fn empty() -> Self::BT;
    fn from_char(c: char) -> Self;
}

impl Block for char {
    type BT = char;
    fn empty() -> Self::BT {
        '.'
    }
    fn from_char(c: char) -> Self::BT {
        c
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Grid {
    width: isize,
    height: isize,
    grid: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(
        width: isize,
        height: isize,
    ) -> Self {
        let mut rows: Vec<Vec<char>> = vec!();
        rows.reserve(usize::try_from(height).unwrap());
        for _ in 0..(height) {
            rows.push(vec![char::empty(); usize::try_from(width).unwrap()]);
        }
        Grid {
            width,
            height,
            grid: rows,
        }
    }
    fn parse_grid_size(input: &str) -> Option<(isize, isize)> {
        let (mut width, mut height) = (0, 0);
        for line in input.lines() {
            if line.is_empty() { continue; }
            height += 1;
            if width == 0 { width = line.chars().count() }
        }
        Some((isize::try_from(width).unwrap(), isize::try_from(height).unwrap()))
    }
    pub fn parse(input: &str) -> Option<Self> {
        let size = Grid::parse_grid_size(&input).unwrap();
        let mut g = Self::new(size.0, size.1);

        let mut y = 0;
        for ln in input.lines() {
            if ln.is_empty() { continue; }
            for (x, c) in ln.chars().enumerate() {
                let pos = Pos::from(isize::try_from(x).unwrap(), isize::try_from(y).unwrap());
                let b = Block::from_char(c);
                g.grid[y][x] = b;
                match c {
                    '.' => {},
                    _ => {},
                }
            }
            y += 1;
        }
        Some(g)
    }
    pub fn neighbors(&self, p: &Pos) -> HashMap<Pos, char> {
        let mut n = HashMap::with_capacity(8);
        for i in -1..=1 {
            for j in -1..=1 {
                let np = Pos::from(p.x + i, p.y + j);
                if *p != np && self.contains(&p) {
                    self.get(&np).and_then(|c| n.insert(np, c));
                }
            }
        }
        n
    }
    pub fn neighbors_xy(&self, p: &Pos) -> HashMap<Pos, char> {
        let mut n = HashMap::with_capacity(4);
        [
            Pos::from(p.x-1, p.y),
            Pos::from(p.x+1, p.y),
            Pos::from(p.x, p.y-1),
            Pos::from(p.x, p.y+1),
        ].into_iter().for_each(|np| {
            self.get(&np).and_then(|c| n.insert(np, c));
        });
        n
    }
    pub fn get(&self, p: &Pos) -> Option<char> {
        if self.contains(p) {
            return Some(self.grid[usize::try_from(p.y).unwrap()][usize::try_from(p.x).unwrap()]);
        }
        None
    }
    pub fn set(&mut self, p: Pos, c: char) {
        self.grid[usize::try_from(p.y).unwrap()][usize::try_from(p.x).unwrap()] = c
    }
    pub fn dist(p1: Pos, p2: Pos) -> usize {
        usize::try_from((p2.x - p1.x).abs() + (p2.y - p1.y).abs())
            .expect("negative dist cannot happen")
    }
    pub fn contains(&self, p: &Pos) -> bool {
        !(p.x < 0 || p.y < 0
        || p.x >= self.width || p.y >= self.height)
    }
    fn str(&self) -> String {
        let mut s = "".to_owned();
        self.grid.iter().for_each(|row| {
            row.iter().for_each(|col| {
                    s.push(*col);
            });
            s.push('\n');
        });
        s
    }
    pub fn print(&self) -> &Self {
        println!("{}", self.str());
        self
    }
    pub fn iter(&self) -> GridIterator {
        GridIterator {
            curr: Pos { x: -1, y: 0 },
            width: self.width,
            height: self.height,
        }
    }
}
