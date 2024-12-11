mod grid;

use std::collections::HashSet;
use std::fs;
use grid::{
    Grid,
    Pos,
};

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let g = grid::Grid::parse(&input).unwrap();
    let r1 = paths(&g);
    let r2 = dpaths(&g);
    println!("{r1} / {r2}");
}

type Path = Vec<Pos>;

fn nx(g: &Grid, path: Path, nc: &char) -> Vec<Path> {
    g.neighbors_xy(&path.last().unwrap())
        .drain()
        .filter(|(_,cx)| cx == nc)
        .map(|(pos,_)| {
            let mut np: Path = path.clone();
            np.push(pos);
            return np;
        })
        .collect()
}

fn paths_from(g: &Grid, p0: Pos) -> Option<Vec<Path>> {
    match g.get(&p0) {
        Some('0') => {
            let paths: Vec<Path> = [vec![p0]].into_iter()//.inspect(|p| println!("0{:?}", p))
                .flat_map(|path| nx(g, path, &'1'))//.inspect(|p| println!("1{:?}", p))
                .flat_map(|path| nx(g, path, &'2'))//.inspect(|p| println!("2{:?}", p))
                .flat_map(|path| nx(g, path, &'3'))//.inspect(|p| println!("3{:?}", p))
                .flat_map(|path| nx(g, path, &'4'))//.inspect(|p| println!("4{:?}", p))
                .flat_map(|path| nx(g, path, &'5'))//.inspect(|p| println!("5{:?}", p))
                .flat_map(|path| nx(g, path, &'6'))//.inspect(|p| println!("6{:?}", p))
                .flat_map(|path| nx(g, path, &'7'))//.inspect(|p| println!("7{:?}", p))
                .flat_map(|path| nx(g, path, &'8'))//.inspect(|p| println!("8{:?}", p))
                .flat_map(|path| nx(g, path, &'9'))//.inspect(|p| println!("9{:?}", p))
                .collect();
            if paths.len() > 0 {
                return Some(paths);
            }
        },
        _ => {},
    }
    None
}

fn paths(g: &Grid) -> usize {
    let mut known = HashSet::new();
    g.iter().flat_map(|p| paths_from(&g, p))
        .flatten()
        .filter(|path|{
            let start = path.first();
            let end = path.last();
            known.insert(format!("{},{}..{},{}",
                start.unwrap().x, start.unwrap().y,
                end.unwrap().x, end.unwrap().y))
        })
        // .inspect(|p| {
        //     for pos in p {
        //         print!("{}:{}{} ", g.get(&pos).unwrap(), pos.x, pos.y);
        //     }
        //     // println!("{:?}", p);
        //     println!("");
        // })
        .count()
}

fn dpaths(g: &Grid) -> usize {
    g.iter().flat_map(|p| paths_from(&g, p))
        .flatten()
        .count()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_smaller() {
    let input = "
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(paths(&g), 2);
}

#[test]
fn test_smaller_d() {
    let input = "
..90..9
...1.98
...2..7
6543456
765.987
876....
987....
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(dpaths(&g), 13);
}

#[test]
fn test_small() {
    let input = "
10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(paths(&g), 3);
}

#[test]
fn test_small_d() {
    let input = "
012345
123456
234567
345678
4.6789
56789.
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(dpaths(&g), 227);
}

#[test]
fn test_mid() {
    let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";
    let g = Grid::parse(input).unwrap();
    assert_eq!(paths(&g), 36);
    assert_eq!(dpaths(&g), 81);
}

}
