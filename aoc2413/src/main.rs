use regex::Regex;
use std::cmp::min;
use std::fs;
use std::ops;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let i = parse(&input);
    let r1 = win_sum(i);
    let r2 = 0;
    println!("{r1} / {r2}");
}

type Size = i64;

#[derive(Clone,Copy,Hash,PartialEq,Debug)]
struct Pos/*ition*/ {
    x: Size,
    y: Size,
}

impl ops::Add for Pos {
    type Output = Pos;
    fn add(self, p: Pos) -> Pos {
        Pos{x: self.x + p.x, y: self.y + p.y}
    }
}

#[derive(Clone,Hash,Debug)]
struct B/*uttons*/ {
    b: char,
    pos: Pos,
    price: Size,
}

#[derive(Clone,Hash,Debug)]
struct S/*olution*/ {
    a: (Size,B),
    b: (Size,B),
}

impl S {
    fn price(&self) -> Size {
        self.a.0 * self.a.1.price + self.b.0 * self.b.1.price
    }
}

#[derive(Clone,Hash,Debug)]
struct G/*ame*/ {
    a: B,
    b: B,
    goal: Pos,
}

/*
 * Button A: X+94, Y+34
 * Button B: X+22, Y+67
 * Prize: X=8400, Y=5400
 */
fn parse(input: &String) -> Vec<G> {
    let re = Regex::new(r"(?ms)^Button A: X\+(\d+), Y\+(\d+)$
^Button B: X\+(\d+), Y\+(\d+)$
^Prize: X=(\d+), Y=(\d+)$
").unwrap();
    re.captures_iter(input)
        // .inspect(|c|println!("{:?}, {:?}", c, 0))
        .map(|c| c.extract())
        .map(|(_,c)| c.map(|c|c.parse::<Size>().unwrap()))
        .map(|[ax,ay,bx,by,gx,gy]| {
            let a = Pos{x:ax, y:ay};
            let b = Pos{x:bx, y:by};

            G {
                a: B{b:'A',pos:a,price:3},
                b: B{b:'B',pos:b,price:1},
                goal:Pos{x:gx, y:gy},
            }
        })
        .collect()
}

fn win_sum(g: impl IntoIterator<Item=G>) -> Size {
    g.into_iter()
        .map(solve)
        .filter(|s| s.is_some())
        .fold(0, |sum,s| sum + s.unwrap().price())
}

const MAX_CLICKS: Size = 100;

fn solve(g: G) -> Option<S> {
    let goal = g.goal;
    let (ax, bx, gx) = (g.a.pos.x, g.b.pos.x, goal.x);
    let (ay, by, gy) = (g.a.pos.y, g.b.pos.y, goal.y);

    let max_a = min(min(gx / ax + 1, gy / ay + 1), MAX_CLICKS);
    let max_b = min(min(gx / bx + 1, gy / by + 1), MAX_CLICKS);

    let mut hits: Vec<S> = vec![];
    for i in 0..max_a {
        for j in 0..max_b {
            let pos = Pos {
                x: i * ax + j * bx,
                y: i * ay + j * by,
            };
            // println!("{},{}", pos.x, pos.y);
            if pos == goal {
                let hit = S {
                    a: (i,g.a.clone()),
                    b: (Size::try_from(j).unwrap(),g.b.clone()),
                };
                hits.push(hit);
            }
        }
    }
    hits.sort_by(|a,b|a.price().cmp(&b.price()));
    if hits.len() > 0 {
        let hit = hits[0].clone();
        return Some(hits[0].clone());
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
".to_string();
        assert_eq!(win_sum(parse(&input)), 280);
    }

    #[test]
    fn test_2() {
        let input = "
Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176
".to_string();
        assert_eq!(win_sum(parse(&input)), 0);
    }

    #[test]
    fn test_3() {
        let input = "
Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450
".to_string();
        assert_eq!(win_sum(parse(&input)), 200);
    }

    #[test]
    fn test_4() {
        let input = "
Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
".to_string();
        assert_eq!(win_sum(parse(&input)), 0);
    }

    #[test]
    fn test_all() {
        let input = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
".to_string();
        assert_eq!(win_sum(parse(&input)), 480);
    }

    #[test]
    fn test_bad() {
        let input = "
Button A: X+87, Y+31
Button B: X+15, Y+36
Prize: X=6672, Y=2500

Button A: X+91, Y+12
Button B: X+13, Y+81
Prize: X=1339, Y=5568
".to_string();
        assert_eq!(win_sum(parse(&input)), 315);
    }
}
