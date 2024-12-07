use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let g = parse(&input).unwrap();
    let c = g.clone().run().unwrap().count();
    let o = obstructions(g);
    println!("{c} / {o}");
}

#[derive(Clone, Copy)]
struct Block {
    c: char,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    grid: Vec<Vec<Block>>,
    pos: Pos,
}

impl Block {
    fn empty() -> Block {
        Block { c: '.', }
    }
    fn from_char(c: char) -> Block {
        Block { c: c, }
    }
    fn value(&self) -> isize {
        match self.c {
            '^' => 1,
            '>' => 1,
            'v' => 1,
            '<' => 1,
            'X' => 1,
            _ => 0,
        }
    }
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
            pos: Pos { x: isize::MAX, y: isize::MAX, },
        }
    }
    fn get_current(&self) -> Block {
        self.get(self.pos.x, self.pos.y)
    }
    fn get(&self, x: isize, y: isize) -> Block {
        self.grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()]
    }
    fn set_current(&mut self, c:char) {
        self.set(self.pos.x, self.pos.y, c);
    }
    fn set(&mut self, x: isize, y: isize, c: char) {
        self.grid[usize::try_from(y).unwrap()][usize::try_from(x).unwrap()].c = c;
    }
    fn calc_next_pos(&self, x: isize, y: isize) -> Pos {
        let new = Pos { x: self.pos.x + x, y: self.pos.y + y };
        if !self.on_grid(new) {
            return Pos { x: isize::MAX, y: isize::MAX };
        }
        new
    }
    fn is_runnable(&self) -> bool {
        self.on_grid(self.pos)
    }
    fn on_grid(&self, p: Pos) -> bool {
        !(p.x < 0 || p.y < 0
        || p.x >= self.width || p.y >= self.height)
    }
    fn set_pos(&mut self, x: isize, y: isize) {
        self.pos = Pos {x: x, y: y };
    }
    fn step(&mut self) -> &Self {
        if !self.is_runnable() {
            return self;
        }
        let mut c = self.get_current().c;
        let mut next = self.pos;
        loop {
            match c {
                '^' => { next = self.calc_next_pos(0, -1); },
                '>' => { next = self.calc_next_pos(1, 0); },
                'v' => { next = self.calc_next_pos(0, 1); },
                '<' => { next = self.calc_next_pos(-1, 0); },
                _ => {},
            }
            if !self.on_grid(next) {
                self.set_current('X');
                self.set_pos(next.x, next.y);
                return self;
            }
            match self.get(next.x, next.y).c {
                '#' => {
                    match c {
                        '^' => { c = '>'; },
                        '>' => { c = 'v'; },
                        'v' => { c = '<'; },
                        '<' => { c = '^'; },
                        _ => {},
                    }
                },
                _ => { break; },
            }
        }
        self.set_current('X');
        self.set_pos(next.x, next.y);
        self.set_current(c);
        self
    }
    fn steps(&mut self, steps: isize) -> &Self {
        for _n in 0..steps {
            self.step();
        }
        self
    }
    fn run(&mut self) -> Option<&Self> {
        let mut visited = HashMap::new();
        while self.is_runnable() {
            let c = &self.get_current().c;
            let mut v: String = visited.entry(self.pos).or_insert(String::from("")).to_string();
            if v.contains(*c) {
                self.print();
                return None;
            }
            v.push(*c);
            visited.insert(self.pos, v);
            self.step();
        }
        Some(self)
    }
    fn count(&self) -> isize {
        self.grid.iter().fold(0, |s1,r| s1 + r.iter().fold(0, |s2, c| s2 + c.value() ))
    }
    fn str(&self) -> String {
        let mut s = "".to_owned();
        self.grid.iter().for_each(|row| {
            row.iter().for_each(|col| s.push(col.c));
            s.push('\n');
        });
        s
    }
    fn print(&self) -> &Self {
        println!("{}", self.str());
        self
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

fn parse(input: &str) -> Option<Grid> {
    let size = parse_grid_size(&input).unwrap();
    let mut g = Grid::new(size.0, size.1);

    let mut y = 0;
    for ln in input.lines() {
        if ln.is_empty() { continue; }
        for (x, b) in ln.chars().enumerate() {
            let ix = isize::try_from(x).unwrap();
            let iy = isize::try_from(y).unwrap();
            g.grid[y][x] = Block::from_char(b);
            match b {
                '^' => g.set_pos(ix, iy),
                '<' => g.set_pos(ix, iy),
                '>' => g.set_pos(ix, iy),
                'v' => g.set_pos(ix, iy),
                _ => {},
            }
        }
        y += 1;
    }
    Some(g)
}

fn obstructions(g: Grid) -> usize {
    let mut found = 0;
    for y in 0..g.width {
        for x in 0..g.height {
            let mut g2 = g.clone();
            let c = g2.get(x, y).c;
            match c {
                '.' => g2.set(x, y, '#'),
                _ => {},
            }
            match g2.run() {
                None => found += 1,
                _ => {},
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_turnturn() {
    let input = "
....#....
.#..^#...
........#
#........
";
    let mut g = parse(input).unwrap();
    assert_eq!(g.width, 9);
    assert_eq!(g.height, 4);
    assert_eq!(g.print().count(), 1);
    assert_eq!(g.run().unwrap().print().count(), 3);
}

#[test]
fn test_run() {
    let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    let mut g = parse(input).unwrap();
    assert_eq!(g.width, 10);
    assert_eq!(g.height, 10);
    assert_eq!(g.print().count(), 1);
    assert_eq!(g.step().print().count(), 2);
    assert_eq!(g.steps(4).print().count(), 6);
    assert_eq!(g.steps(4).print().count(), 10);
    assert_eq!(g.steps(5).print().count(), 15);
    assert_eq!(g.steps(6).print().count(), 20);
    assert_eq!(g.run().unwrap().print().count(), 41);
}

#[test]
fn test_obstructions() {
    let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    let g = parse(input).unwrap();
    assert_eq!(g.width, 10);
    assert_eq!(g.height, 10);

    assert_eq!(obstructions(g), 6);
}

}
