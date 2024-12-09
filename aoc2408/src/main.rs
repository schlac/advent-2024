use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let g = Grid::parse(&input).unwrap();
    let r1 = antinodes(g.clone());
    let r2 = more_antinodes(g);
    println!("{r1} / {r2}");
}

struct GridIterator {
    curr: Pos,
    width: isize,
    height: isize,
}

impl Iterator for GridIterator {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = (self.curr.x, self.curr.y);
        if x + 1 < self.width {
            self.curr = Pos { x: x+1, y: y };
            return Some(self.curr);
        } else if y + 1 < self.height {
            self.curr = Pos { x: 0, y: y+1 };
            return Some(self.curr);
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{{{},{}}}'", self.x, self.y)
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Block {
    c: char,
}

impl Block {
    fn empty() -> Block {
        Block { c: '.', }
    }
    fn from_char(c: char) -> Block {
        Block { c: c, }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{}'", self.c)
    }
}

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    grid: Vec<Vec<Block>>,
    sigs: HashMap<Block, Vec<Pos>>,
}

impl Grid {
    fn new(
        width: isize,
        height: isize,
    ) -> Self {
        let mut rows: Vec<Vec<Block>> = vec!();
        rows.reserve(usize::try_from(height).unwrap());
        for _ in 0..(height) {
            rows.push(vec![Block::empty(); usize::try_from(width).unwrap()]);
        }
        Grid {
            width: width,
            height: height,
            grid: rows,
            sigs: HashMap::new(),
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
    fn parse(input: &str) -> Option<Self> {
        let size = Grid::parse_grid_size(&input).unwrap();
        let mut g = Self::new(size.0, size.1);

        let mut y = 0;
        for ln in input.lines() {
            if ln.is_empty() { continue; }
            for (x, c) in ln.chars().enumerate() {
                let pos = Pos {
                    x: isize::try_from(x).unwrap(),
                    y: isize::try_from(y).unwrap(),
                };
                let b = Block::from_char(c);
                g.grid[y][x] = b;
                match c {
                    '.' => {},
                    '#' => {},
                    _ => {
                        g.sigs.entry(b).or_insert(Vec::new()).push(pos);
                    },
                }
            }
            y += 1;
        }
        Some(g)
    }
    fn get(&self, x: isize, y: isize) -> Block {
        self.grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()]
    }
    fn set(&mut self, x: isize, y: isize, c: char) {
        self.grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].c = c;
    }
    fn dist(&self, p1: Pos, p2: Pos) -> isize {
        (p2.x - p1.x).abs() + (p2.y - p1.y).abs()
    }
    fn on_grid(&self, p: Pos) -> bool {
        !(p.x < 0 || p.y < 0
        || p.x >= self.width || p.y >= self.height)
    }
    fn str(&self) -> String {
        let mut s = "".to_owned();
        self.grid.iter().for_each(|row| {
            row.iter().for_each(|col| {
                    s.push(col.c);
            });
            s.push('\n');
        });
        s
    }
    fn print(&self) -> &Self {
        println!("{}", self.str());
        self
    }
    fn iter(&self) -> GridIterator {
        GridIterator { curr: Pos { x: -1, y: 0 }, width: self.width, height: self.height }
    }
}

fn in_line(p1: Pos, p2: Pos, p3: Pos) -> bool {
    let dxc = p1.x - p2.x;
    let dyc = p1.y - p2.y;

    let dxl = p3.x - p2.x;
    let dyl = p3.y - p2.y;

    let cross = dxc * dyl - dyc * dxl;
    cross == 0
}

fn antinodes(g: Grid) -> usize {
    let mut nodes: HashSet<Pos> = HashSet::new();
    for (sig, pos) in g.sigs.iter() {
        println!("Caluclating signal {}", sig);
        let p = pos.len();
        for n in 0..p {
            for m in 0..n {
                let (npos, mpos) = (pos[n], pos[m]);
                for p in g.iter() {
                    if p == npos || p == mpos { continue; }
                    if !in_line(npos, mpos, p) { continue; }
                    // println!("-- n={n},m={m},p={p}");
                    let d1 = g.dist(p, npos);
                    let d2 = g.dist(p, mpos);

                    if d1 == 2*d2 || d2 == 2*d1 {
                        nodes.insert(p);
                        println!("--- n={n},m={m},p={p}");
                    }
                }
            }
        }
    }
    nodes.len()
}

fn more_antinodes(g: Grid) -> usize {
    let mut nodes: HashSet<Pos> = HashSet::new();
    for (sig, pos) in g.sigs.iter() {
        println!("Caluclating signal {}", sig);
        let len = pos.len();
        if len > 1 {
            for p in pos {
                nodes.insert(*p);
            }
        }
        for n in 0..len {
            for m in 0..n {
                let (npos, mpos) = (pos[n], pos[m]);
                for p in g.iter() {
                    if !in_line(npos, mpos, p) { continue; }
                    nodes.insert(p);
                    println!("--- n={n},m={m},p={p}");
                }
            }
        }
    }
    nodes.len()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_small() {
    let input = "
.a.......
.a.......
......bb.
.........
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(g.width, 9);
    assert_eq!(g.height, 4);

    assert_eq!(antinodes(g), 3);
}

#[test]
fn test_mid() {
    let input = "
..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(g.width, 10);
    assert_eq!(g.height, 10);

    assert_eq!(antinodes(g), 2);
}

#[test]
fn test_run() {
    let input = "
##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(g.width, 12);
    assert_eq!(g.height, 12);

    assert_eq!(antinodes(g.clone()), 14);
    assert_eq!(more_antinodes(g), 34);
}

#[test]
fn test_run2() {
    let input = "
T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
";
let g = Grid::parse(input).unwrap();
assert_eq!(g.width, 10);
assert_eq!(g.height, 10);

assert_eq!(more_antinodes(g), 9);
}

}
