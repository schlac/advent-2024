mod grid;

use regex::Regex;
use std::fs;
use grid::{Pos, Grid};

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let grid = Grid::new(101, 103);
    let dudes = parse(&input);
    let mut r = Room {
        grid, dudes, round: 0
    };
    r.run(100);
    println!("{} / {}", r.safety(), 0);
}

/*
 * p=0,4 v=3,-3
 */
fn parse(input: &String) -> Vec<Dude> {
    let re = Regex::new(r"p=(\d+),(\d+) v=([\d-]+),([\d-]+)").unwrap();
    // println!("{:?}", re);
    re.captures_iter(input)
        // .inspect(|c|println!("read {:?}", c))
        .map(|c| c.extract())
        .map(|(_,c)| c.map(|c|c.parse::<Size>().unwrap()))
        .map(|[x,y,vx,vy]| {
            Dude {
                pos: Pos { x, y },
                v: Pos { x:vx, y:vy },
            }
        })
        .collect()
}

type Size = isize;

#[derive(Clone,Hash,Debug)]
struct Dude {
    pos: Pos,
    v: Pos,
}

#[derive(Clone,Hash,Debug)]
struct Area {
    from: Pos,
    to: Pos,
}

impl Area {
    fn contains(&self, pos: Pos) -> bool {
        self.from.x <= pos.x &&
        self.to.x >= pos.x &&
        self.from.y <= pos.y &&
        self.to.y >= pos.y
    }
}

#[derive(Clone,Hash,Debug)]
struct Room {
    grid: Grid,
    dudes: Vec<Dude>,
    round: usize,
}

impl Room {
    fn safety(self) -> usize {
        let areas = [
            Area {
                from: Pos{x: 0, y: 0},
                to: Pos{x: self.grid.width / 2 - 1, y: self.grid.height / 2 - 1},
            },
            Area {
                from: Pos{x: self.grid.width / 2 + 1, y: 0},
                to: Pos{x: self.grid.width, y: self.grid.height / 2 - 1},
            },
            Area {
                from: Pos{x: 0, y: self.grid.height / 2 + 1},
                to: Pos{x: self.grid.width / 2 - 1, y: self.grid.height},
            },
            Area {
                from: Pos{x: self.grid.width / 2 + 1, y: self.grid.height / 2 + 1},
                to: Pos{x: self.grid.width, y: self.grid.height},
            },
        ];
        let mut n = [0,0,0,0];
        // println!("dudes: {:?}", self.dudes);
        // println!("areas {:?}", areas);
        for dude in self.dudes {
            // println!("dude: {:?} {:?}", dude.pos, areas[2].contains(dude.pos));
            for i in 0..4 {
                if areas[i].contains(dude.pos) {
                    n[i] = n[i] + 1;
                    break;
                }
            }
        }
        n.iter()
            // .inspect(|n| print!("{} ", n))
            .fold(1, |s, n| s * n)
    }
    fn run(&mut self, steps: usize) {
        for _ in 0..steps {
            let dudes = self.dudes.clone();
            self.dudes = dudes.into_iter().map(|mut dude|{
                let mut x = dude.pos.x + dude.v.x;
                if x < 0 { x = x + self.grid.width; }
                if x >= self.grid.width { x = x - self.grid.width; }
                let mut y = dude.pos.y + dude.v.y;
                if y < 0 { y = y + self.grid.height; }
                if y >= self.grid.height { y = y - self.grid.height; }

                dude.pos = Pos{x, y};
                dude
            }).collect();
        }
    }
    fn is_tree(self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
".to_string();
        let grid = Grid::new(11, 7);
        let dudes = parse(&input);
        let mut r = Room {
            grid, dudes, round: 0
        };
        r.run(100);

        assert_eq!(r.clone().safety(), 12);
        assert_eq!(r.is_tree(), false);
    }

    #[test]
    fn test_tree() {
        let input = "
p=5,0 v=0,0
p=4,1 v=0,0
p=6,1 v=0,0
p=3,2 v=0,0
p=7,2 v=0,0
p=2,3 v=0,0
p=8,3 v=0,0
p=5,4 v=0,0
p=5,5 v=0,0
p=5,6 v=0,0
".to_string();
        let grid = Grid::new(11, 7);
        let dudes = parse(&input);
        let mut r = Room {
            grid, dudes, round: 0
        };
        r.run(100);

        assert_eq!(r.is_tree(), true);
    }

}
