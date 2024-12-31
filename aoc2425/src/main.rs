use LKType::*;
use itertools::Itertools;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let d = parse(&input)
        .expect("Parse failed");
    println!("{} / l{} k{}", d.fit().count(), d.locks.len(), d.keys.len());
}

fn parse(input: &String) -> Option<Data> {
    let mut d = Data::new();

    let mut lk_str = Vec::<String>::with_capacity(KEY_HEIGHT);
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        match line.chars().next().expect("no char")
        {
            '#' | '.' => {
                lk_str.push(line.to_string());
            },
            _ => { continue },
        }
        if lk_str.len() >= KEY_HEIGHT + 2 {
            // println!("{:?}", lk_str);
            let mut seq = [0, 0, 0, 0, 0];
            for i in 1..=KEY_HEIGHT {
                for (pos, c) in lk_str[i].chars().enumerate() {
                    match c {
                        '#' => seq[pos] += 1,
                        _ => continue,
                    }
                }
            }
            match lk_str[0].chars().next().expect("no char") {
                '#' => d.locks.push(LK::new(Lock, seq)),
                '.' => d.keys.push(LK::new(Key, seq)),
                _ => panic!(),
            }
            lk_str.clear();
        }
    }
    // println!("{}", d);
    Some(d)
}

type Pair = (LK, LK);

struct Data {
    locks: Vec<LK>,
    keys: Vec<LK>,
}

impl Data {
    fn new() -> Self {
        Data {
            locks: vec![],
            keys: vec![],
        }
    }
    fn pairs(&self) -> impl Iterator<Item=Pair> {
        self.locks.clone().into_iter()
            .cartesian_product(self.keys.clone().into_iter()).into_iter()
    }
    fn fit(&self) -> impl Iterator<Item=Pair> {
        self.pairs().filter(|(l,k)| l.seq & k.seq == 0)
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "locks:    |-5-| |-4-| |-3-| |-2-| |-1-|")?;
        self.locks.iter().for_each(|l| writeln!(fmt, "  {}", l).expect("lock err"));
        writeln!(fmt, "keys:     |-5-| |-4-| |-3-| |-2-| |-1-|")?;
        self.keys.iter().for_each(|k| writeln!(fmt, "  {}", k).expect("key err"));
        Result::Ok(())
    }
}

#[derive(Clone,Copy,Hash,Debug,PartialEq,Eq)]
enum LKType {
    Lock,
    Key,
}

const KEY_LENGTH: usize = 5;
const KEY_HEIGHT: usize = 5;
type StoreType = i32;
type Depth = u8;

#[derive(Clone,Copy,Hash,Debug,PartialEq,Eq)]
struct LK {
    t: LKType,
    seq: StoreType,
}

impl LK {
    fn new(t: LKType, v: [Depth; KEY_LENGTH]) -> Self {
        let mut seq: StoreType = 0;
        for (i, d) in v.iter().enumerate() {
            let d = *d as u32;
            match t {
                Lock => {
                    let x = (2_i32.pow(d) - 1) << (i * (KEY_HEIGHT + 1));
                    match d as usize {
                        y@1 | y@2 | y@3 | y@4 => seq |= x << (KEY_HEIGHT - y),
                        5 => seq |= 0b11111 << (i * (KEY_HEIGHT + 1)),
                        _ => {},
                    }
                },
                Key => {
                    seq |= (2_i32.pow(d) - 1) << (i * (KEY_HEIGHT + 1));
                },
            }
            // println!("{} {}  {:032b}", i, d, seq);
        }
        match t {
            Lock => LK { t, seq, },
            Key => LK { t, seq, },
        }
    }
}

impl std::fmt::Display for LK {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self.t {
            Key => write!(fmt,  "key  ")?,
            Lock => write!(fmt, "lock ")?,
        }
        write!(fmt, "{:032b}", self.seq)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_fit() {
        let input = "
#####
.####
.####
.####
.#.#.
.#...
.....

.....
.....
.....
#....
#.#..
#.#.#
#####
".to_string();
        let d = parse(&input)
            .expect("Parse failed");
        assert_eq!(d.locks.len(), 1);
        assert_eq!(d.keys.len(), 1);
        assert_eq!(d.pairs().count(), 1);
        assert_eq!(d.fit().count(), 1);
    }

    #[test]
    fn test_small_overlap() {
        let input = "
#####
#####
.####
..#.#
....#
....#
.....

.....
.....
#..#.
##.#.
####.
####.
#####
".to_string();
        let d = parse(&input)
            .expect("Parse failed");
        assert_eq!(d.locks.len(), 1);
        assert_eq!(d.keys.len(), 1);
        assert_eq!(d.pairs().count(), 1);
        assert_eq!(d.fit().count(), 0);
    }

    #[test]
    fn test() {
        let input = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
".to_string();
        let d = parse(&input)
            .expect("Parse failed");
        assert_eq!(d.locks.len(), 2);
        assert_eq!(d.keys.len(), 3);
        assert_eq!(d.pairs().count(), 6);
        assert_eq!(d.fit().count(), 3);
        assert_eq!(d.fit().fold("".to_string(),
            |s,(k,l)| s + "\n" + &k.to_string() + "\n" + &l.to_string()), "
lock 00011100011110011100011111000000
key  00000001000000000011000000000111
lock 00011100011111000000011000010000
key  00000011000000001111000111001111
lock 00011100011111000000011000010000
key  00000001000000000011000000000111");
    }
}
