mod grid;

use Direction::*;
use grid::{Pos, Grid};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
    rc::Rc,
};

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut grid = parse(&input)
        .expect("Parse failed");
    grid.solve();
    println!("{} / {}", grid.cost_to_end().expect("not solved"), 0);
}

fn parse(input: &String) -> Option<Maze> {
    let mut grid_lines = String::new();

    for (_, line) in input.lines().enumerate() {
        if line.is_empty() { continue; }
        match line.chars().next()?
        {
            '#' => {
                grid_lines.push_str(line);
                grid_lines.push('\n');
            },
            _ => {},
        }
    }
    Some(Maze::parse(&grid_lines))
}

#[derive(Clone,Copy,Hash,Debug,PartialEq,Eq)]
enum Cost {
    STEP = 1,
    TURN = 1000,
}

#[derive(Clone,Copy,Hash,Debug,PartialEq,Eq)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let d =
        match self {
            NORTH => '^', EAST => '>',
            SOUTH => 'v', WEST => '<',
        };
        write!(fmt, "{}", d)
    }
}

impl Direction {
    fn values() -> [Direction; 4] {
        [ EAST, SOUTH, WEST, NORTH, ]
    }
    fn pos(&self, pos: &Pos) -> Pos {
        let mut new = Pos { ..*pos };
        match self {
            NORTH => { new.y -= 1 },
            EAST => { new.x += 1 },
            WEST => { new.x -= 1 },
            SOUTH => { new.y += 1 },
        }
        new
    }
    fn turn(&self, to: &Direction) -> usize {
        match self {
            s if s == to => { return 0; },
            NORTH | SOUTH => {
                match to {
                    NORTH | SOUTH => { return 2 * (Cost::TURN as usize) },
                    EAST | WEST => { return Cost::TURN as usize },
                }
            },
            EAST | WEST => {
                match to {
                    NORTH | SOUTH => { return Cost::TURN as usize },
                    EAST | WEST => { return 2 * (Cost::TURN as usize) },
                }
            },
        }
    }
    fn from_pos(f: &Pos, t: &Pos) -> Direction {
        let dx = f.x - t.x;
        let dy = f.y - t.y;
        match dx {
            0 => match dy {
                0.. => { NORTH },
                ..0 => { SOUTH },
            },
            0.. => { WEST },
            ..0 => { EAST },
        }
    }
    fn cost(&self, p: &Pos, c: &Pos, newd: Option<&mut Direction>) -> usize {
        let d2 = Direction::from_pos(p, c);
        if newd.is_some() {
            *(newd.unwrap()) = d2;
        }
        self.turn(&d2) + Cost::STEP as usize
    }
}

type RCell = Rc<RefCell<Cell>>;

#[derive(Clone,Hash,Debug,PartialEq,Eq)]
struct Path {
    start_direction: Direction,
    cells: Vec<Pos>,
}

impl Path {
    fn new(start: Pos, start_direction: Direction) -> Self {
        Self { cells: vec![start], start_direction }
    }
    fn extend(&self, next: Pos) -> Self {
        let mut p = self.clone();
        p.cells.push(next);
        p
    }
    fn cost(&self) -> usize {
        let mut cells = self.cells.iter();
        let mut s = 0;
        let mut d = self.start_direction;
        let mut p = cells.next().unwrap();
        for c in cells {
            s += d.clone().cost(p, c, Some(&mut d));
            p = c;
        }
        s
    }
    fn print(&self) {
        let mut cells = self.cells.iter();
        let mut s = 0;
        let mut d = self.start_direction;
        let mut p = cells.next().unwrap();
        for c in cells {
            s += d.clone().cost(p, c, Some(&mut d));
            p = c;
            print!("{}", d);
        }
        println!(" cost: {}", s);
    }
}

#[derive(Clone,Debug)]
struct Cell {
    pos: Pos,
    paths: HashSet<Path>,
    best_cost: usize,
}

impl Cell {
    fn new(pos: Pos) -> Self {
        Cell {
            pos,
            paths: HashSet::new(),
            best_cost: usize::MAX - (Cost::STEP as usize) - (Cost::TURN as usize),
        }
    }
    fn new_d(&self, d: Direction) -> Self {
        Self::new(d.pos(&self.pos))
    }
    fn edge_cost(&self) -> usize {
        self.best_cost + (Cost::STEP as usize) + (Cost::TURN as usize)
    }
}

#[derive(Clone,Debug)]
struct Maze {
    nr_cells: usize,
    start: Pos,
    start_direction: Direction,
    end: Pos,
    grid: Grid,
    cells: HashMap<Pos, RCell>,
}

impl Maze {
    fn parse(input: &String) -> Self {
        let grid = Grid::parse(input)
                .expect("parse failed");
        let nr_cells = grid.iter().fold(0, |s,(_,c)| match c { '.' | 'S' | 'E' => 1, _ => 0, } + s);
        Maze {
            nr_cells,
            start: Pos { x: 1, y: (&grid).height - 2 },
            start_direction: EAST,
            end: Pos { x: (&grid).width - 2, y: 1 },
            cells: HashMap::with_capacity(usize::try_from(
                (&grid).width * (&grid).height / 2).expect("bad size")),
            grid,
        }
    }
    fn solve(&mut self) {
        let mut cell = Cell::new(self.start);
        cell.paths.insert(Path::new(self.start, self.start_direction));
        self.cells.insert(self.start, Rc::new(RefCell::new(cell)));
        self.find_paths();
    }
    fn find_paths(&mut self) {
        let mut i: u16 = 5000;
        let mut todo = HashSet::new();
        todo.insert(self.start);
        while !todo.is_empty() {
            // println!("cells\n{:?}", &self.cells);
            let from_pos = todo.iter().last().copied().unwrap();
            todo.remove(&from_pos);
            if i >= 5000 {
                i = 0;
                println!("\nTODO: {}\nVISITED: {}/{}\nNEXT: {:?}",
                    todo.len(), self.cells.len(), self.nr_cells, from_pos);
            }
            i += 1;
            let mut new_cells = vec![];
            {
                let from_cell = self.cells.get(&from_pos).unwrap().borrow();
                for d in Direction::values() {
                    let mut cell = from_cell.new_d(d);
                    let pos = cell.pos;
                    // println!("pp {:?} -{:?}-> {:?} pp", from_pos, d, pos);
                    match (&self.grid).get(&pos).unwrap() {
                        '.' | 'S' | 'E' => {},
                        _ => { continue; },
                    }
                    let cell_best = cell.best_cost;
                    from_cell.paths.iter().for_each(|p| {
                        // println!("++ {:?} ++ {:?}", p, pos);
                        if !p.cells.contains(&pos) {
                            let new_path = p.extend(pos);
                            let new_cost = new_path.cost();
                            if new_cost <= cell.edge_cost() {
                                if new_cost < cell.best_cost {
                                    cell.best_cost = new_cost;
                                }
                                cell.paths.insert(new_path);
                            }
                        }
                    });
                    if cell_best != cell.best_cost {
                        let edge_cost = cell.edge_cost();
                        cell.paths.retain(|p|p.cost() <= edge_cost);
                    }
                    // println!();
                    if self.cells.contains_key(&pos) {
                        let mut new = false;
                        let ecell = &mut self.cells.get(&pos).unwrap().borrow_mut();
                        let ecell_best = ecell.best_cost;
                        cell.paths.into_iter().for_each(|p| {
                            new |= ecell.paths.insert(p);
                        });
                        if new {
                            if ecell_best != ecell.best_cost {
                                let edge_cost = ecell.edge_cost();
                                ecell.paths.retain(|p|p.cost() <= edge_cost);
                            }
                            todo.insert(ecell.pos);
                        }
                    } else {
                        let rcell = Rc::new(RefCell::new(cell));
                        new_cells.push((pos, rcell));
                    }
                }
            }
            for (pos, rcell) in new_cells {
                self.cells.insert(pos, rcell);
                todo.insert(pos);
            }

            // println!("-------------------------------------");
       }
    }
    fn cost_to(&self, pos: &Pos) -> Option<usize> {
        let e = self.cells.get(pos)?.borrow();
        let mut paths: Vec<Path> = e.paths.clone().into_iter().collect();
        paths.sort_by(|a,b| a.cost().cmp(&b.cost()));
        // paths.iter().for_each(|p| p.print());
        paths.get(0).and_then(|p| Some(p.cost()))
    }
    fn cost_to_end(&self) -> Option<usize> {
        self.cost_to(&self.end)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path0() {
        let p0 = Path::new(Pos { x: 1, y: 1 }, EAST);
        assert_eq!(p0.cost(), 0);
    }

    #[test]
    fn path1() {
        let p = Path{ cells: [Pos { x: 1, y: 1 }, Pos { x: 2, y: 1 }].to_vec(), start_direction: EAST, };
        assert_eq!(p.cost(), 1);
    }

    #[test]
    fn path2() {
        let p = Path{ cells: [Pos { x: 1, y: 1 }, Pos { x: 1, y: 2 }].to_vec(), start_direction: EAST, };
        assert_eq!(p.cost(), 1001);
    }

    #[test]
    fn path3() {
        let p = Path{ cells: [Pos { x: 1, y: 1 }, Pos { x: 0, y: 1 }].to_vec(), start_direction: EAST, };
        assert_eq!(p.cost(), 2001);
    }

    #[test]
    fn path4() {
        let p = Path{ cells: [Pos { x: 1, y: 1 }, Pos { x: 0, y: 1 }, Pos { x: 0, y: 2 }].to_vec(), start_direction: EAST, };
        assert_eq!(p.cost(), 3002);
    }

    #[test]
    fn test_smallest() {
        let input = "
###
#E#
#S#
###
".to_string();
        let mut grid = parse(&input)
            .expect("Parse failed");
        grid.solve();
        assert_eq!(grid.cost_to_end().expect("not solved"), 1001);
    }

    #[test]
    fn test_smallest_plus() {
        let input = "
###
#E#
#.#
#S#
###
".to_string();
        let mut grid = parse(&input)
            .expect("Parse failed");
        grid.solve();
        assert_eq!(grid.cost_to_end().expect("not solved"), 1002);
    }

    #[test]
    fn test_options() {
        let input = "
#####
#..E#
#S..#
#####
".to_string();
        let mut grid = parse(&input)
            .expect("Parse failed");
        grid.solve();
        assert_eq!(grid.cost_to_end().expect("not solved"), 1003);
    }

    #[test]
    fn test_small_alt() {
        let input = "
########
#....#E#
#.#..#.#
#.#..#.#
#S#....#
########
".to_string();
        let mut grid = parse(&input)
            .expect("Parse failed");
        grid.solve();
        assert_eq!(grid.cost_to_end().expect("not solved"), 5014);
    }

    #[test]
    fn test_loop() {
        let input = "
#############
#..........E#
###.#.#####.#
#...#.....#.#
#.#.#.###.#.#
#.....#...#.#
#.###.#.#.#.#
#S..#.....#.#
#############
".to_string();
        let mut grid = parse(&input)
            .expect("Parse failed");
        grid.solve();
        assert_eq!(grid.cost_to_end().expect("not solved"), 4016);
    }

    #[test]
    fn test() {
        let input = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
".to_string();
        let mut grid = parse(&input)
            .expect("Parse failed");
        grid.solve();
        assert_eq!(grid.cost_to_end().expect("not solved"), 7036);
    }

    #[test]
    fn test_bigger() {
        let input = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
".to_string();
        let mut grid = parse(&input)
            .expect("Parse failed");
        grid.solve();
        assert_eq!(grid.cost_to_end().expect("not solved"), 11048);
    }
}
