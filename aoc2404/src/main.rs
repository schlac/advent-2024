use std::cmp;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let c = count_xmas(&input);
    let m = count_mas(&input);
    println!("{c} / {m}");
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct P {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Xmas {
    x: P,
    m: P,
    a: P,
    s: P,
}

impl Xmas {
    fn from_usize(x: [usize; 2], m: [usize; 2], a: [usize; 2], s: [usize; 2]) -> Self {
        Xmas {
            x: P{x: u32::try_from(x[0]).unwrap(), y: u32::try_from(x[1]).unwrap()},
            m: P{x: u32::try_from(m[0]).unwrap(), y: u32::try_from(m[1]).unwrap()},
            a: P{x: u32::try_from(a[0]).unwrap(), y: u32::try_from(a[1]).unwrap()},
            s: P{x: u32::try_from(s[0]).unwrap(), y: u32::try_from(s[1]).unwrap()},
        }
    }

    fn is_valid_p(&self, c: char, p: P) -> bool {
        if [self.x, self.m, self.a, self.s].contains(&p) {
            match c {
            'X' => { return self.x == p; },
            'M' => { return self.m == p; },
            'A' => { return self.a == p; },
            'S' => { return self.s == p; },
            _ => { return false; },
            }
        }
        return true;
    }

    fn is_valid(&self, c: char, x: u32, y: u32) -> bool {
        self.is_valid_p(c, P{x: x, y: y})
    }
}

fn prune_xmas(v: Vec<Xmas>, c: char, x: u32, y: u32) -> Vec<Xmas> {
    v.into_iter().filter(|xmas| {
        let v = xmas.is_valid(c, u32::try_from(x).unwrap(), u32::try_from(y).unwrap());
        // if !v { println!("remove {c},{x},{y}: {xmas:?}"); }
        v
    }).collect()
}

fn count_xmas(input: &str) -> usize {
    let mut words: Vec<Xmas> = vec![];
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    for (y, l) in input.lines().enumerate() {
        max_y = cmp::max(max_y, u32::try_from(y).unwrap());
        for (x, c) in l.chars().enumerate() {
            max_x = cmp::max(max_x, u32::try_from(x).unwrap());
            words = prune_xmas(words, c, u32::try_from(x).unwrap(), u32::try_from(y).unwrap());

            match c {
                'X' => {
                    words.push(Xmas::from_usize([x, y], [x+1, y], [x+2, y], [x+3, y]));
                    words.push(Xmas::from_usize([x, y], [x+1, y+1], [x+2, y+2], [x+3, y+3]));
                    words.push(Xmas::from_usize([x, y], [x, y+1], [x, y+2], [x, y+3]));

                    if x >= 3 {
                    words.push(Xmas::from_usize([x, y], [x-1, y+1], [x-2, y+2], [x-3, y+3]));
                    }
                },
                'S' => {
                    words.push(Xmas::from_usize([x+3, y], [x+2, y], [x+1, y], [x, y]));
                    words.push(Xmas::from_usize([x+3, y+3], [x+2, y+2], [x+1, y+1], [x, y]));
                    words.push(Xmas::from_usize([x, y+3], [x, y+2], [x, y+1], [x, y]));

                    if x >= 3 {
                    words.push(Xmas::from_usize([x-3, y+3], [x-2, y+2], [x-1, y+1], [x, y]));
                    }
                },
                _ => {},
            }
        }
        words = prune_xmas(words, '1', max_x+1, u32::try_from(y).unwrap());
    }
    for x in 0..max_x+1 {
        words = prune_xmas(words, '2', x, max_y+1);
    }
    words = prune_xmas(words, '3', max_x+1, max_y+1);
    // println!("max {max_x},{max_y}");
    // for xmas in words.iter() {
    //     println!("has {xmas:?}");
    // }
    words.len()
}

#[derive(Debug)]
struct Mas {
    m1: P,
    m2: P,
    a: P,
    s1: P,
    s2: P,
}

impl Mas {
    fn from_usize(m1: [usize; 2], m2: [usize; 2], a: [usize; 2], s1: [usize; 2], s2: [usize; 2]) -> Self {
        Mas {
            m1: P{x: u32::try_from(m1[0]).unwrap(), y: u32::try_from(m1[1]).unwrap()},
            m2: P{x: u32::try_from(m2[0]).unwrap(), y: u32::try_from(m2[1]).unwrap()},
            a: P{x: u32::try_from(a[0]).unwrap(), y: u32::try_from(a[1]).unwrap()},
            s1: P{x: u32::try_from(s1[0]).unwrap(), y: u32::try_from(s1[1]).unwrap()},
            s2: P{x: u32::try_from(s2[0]).unwrap(), y: u32::try_from(s2[1]).unwrap()},
        }
    }

    fn is_valid_p(&self, c: char, p: P) -> bool {
        if [self.m1, self.m2, self.a, self.s1, self.s2].contains(&p) {
            match c {
            'M' => { return self.m1 == p || self.m2 == p; },
            'A' => { return self.a == p; },
            'S' => { return self.s1 == p || self.s2 == p; },
            _ => { return false; },
            }
        }
        return true;
    }

    fn is_valid(&self, c: char, x: u32, y: u32) -> bool {
        self.is_valid_p(c, P{x: x, y: y})
    }
}

fn prune_mas(v: Vec<Mas>, c: char, x: u32, y: u32) -> Vec<Mas> {
    v.into_iter().filter(|xmas| {
        let v = xmas.is_valid(c, u32::try_from(x).unwrap(), u32::try_from(y).unwrap());
        // if !v { println!("remove {c},{x},{y}: {xmas:?}"); }
        v
    }).collect()
}

fn count_mas(input: &str) -> usize {
    let mut v: Vec<Mas> = vec![];
    let mut max_x: u32 = 0;
    let mut max_y: u32 = 0;
    for (y, l) in input.lines().enumerate() {
        max_y = cmp::max(max_y, u32::try_from(y).unwrap());
        for (x, c) in l.chars().enumerate() {
            max_x = cmp::max(max_x, u32::try_from(x).unwrap());
            v = prune_mas(v, c, u32::try_from(x).unwrap(), u32::try_from(y).unwrap());

            match c {
                'M' => {
                    v.push(Mas::from_usize([x, y], [x+2, y], [x+1, y+1], [x, y+2], [x+2, y+2]));
                    v.push(Mas::from_usize([x, y], [x, y+2], [x+1, y+1], [x+2, y], [x+2, y+2]));
                },
                'S' => {
                    v.push(Mas::from_usize([x, y+2], [x+2, y+2], [x+1, y+1], [x, y], [x+2, y]));
                    v.push(Mas::from_usize([x+2, y], [x+2, y+2], [x+1, y+1], [x, y], [x, y+2]));
                },
                _ => {},
            }
        }
        v = prune_mas(v, '1', max_x+1, u32::try_from(y).unwrap());
    }
    for x in 0..max_x+1 {
        v = prune_mas(v, '2', x, max_y+1);
    }
    v = prune_mas(v, '3', max_x+1, max_y+1);
    // println!("max {max_x},{max_y}");
    // for xmas in v.iter() {
    //     println!("has {xmas:?}");
    // }
    v.len()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test1f() {
    let input = "XMAS";
    assert_eq!(count_xmas(input), 1);
}

#[test]
fn test1b() {
    let input = "SAMX";
    assert_eq!(count_xmas(input), 1);
}

#[test]
fn test1d() {
    let input = "X\nM\nA\nS";
    assert_eq!(count_xmas(input), 1);
}

#[test]
fn test1u() {
    let input = "S\nA\nM\nX";
    assert_eq!(count_xmas(input), 1);
}

#[test]
fn test1ud() {
    let input =
"X..S
.MA.
.MA.
X..S
";
    assert_eq!(count_xmas(input), 2);
}

#[test]
fn test1du() {
    let input = "
S..X
.AM.
.AM.
S..X";
    assert_eq!(count_xmas(input), 2);
}

#[test]
fn test_xmas() {
    let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    assert_eq!(count_xmas(input), 18);
}

#[test]
fn test_mas1() {
    let input = "
M.M
.A.
S.S";
    assert_eq!(count_mas(input), 1);
}

#[test]
fn test_mas2() {
    let input = "
M.S
.A.
M.S";
    assert_eq!(count_mas(input), 1);
}

#[test]
fn test_mas3() {
    let input = "
S.S
.A.
M.M";
    assert_eq!(count_mas(input), 1);
}

#[test]
fn test_mas4() {
    let input = "
S.M
.A.
S.M";
    assert_eq!(count_mas(input), 1);
}

#[test]
fn test_mas() {
    let input = "
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
    assert_eq!(count_mas(input), 9);
}

}
