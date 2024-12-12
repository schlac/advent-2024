use std::fs;
use std::collections::HashMap;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let mut o = parse(&input).unwrap();
    let r1 = blink(25, &mut o);
    println!("r1: {r1}");
    let r2 = blink(50, &mut o);
    println!("r2: {r2}");
}

type N = u64;
type Stone = u64;
type Stones = HashMap<Stone, N>;

fn parse(input: &str) -> Option<Stones> {
    Some(input.lines()
        .filter(|l| !l.is_empty())
        .fold(Stones::new(), |o, l| l.split_whitespace()
            .fold(o, |mut o, s| {
                let s = s.parse().unwrap();
                o.entry(s).and_modify(|n| *n += 1).or_insert(1);
                o
            })))
}

fn sum(stones: &Stones) -> usize {
    stones.values().sum::<N>() as usize
}

fn blink(n: usize, stones: &mut Stones) -> usize {
    for _ in 0..n {
        *stones = blink1(stones);
    }
    sum(stones)
}

fn blink1(stones: &Stones) -> Stones {
    // println!("next round:");
    let mut new_stones = Stones::with_capacity(stones.len() * 2);
    for (stone, n) in stones {
        blink_rules(&stone).into_iter().for_each(|s| {
            new_stones.entry(s).and_modify(|m| *m += n).or_insert(*n);
        });
    }
    new_stones
}

fn blink_rules(s: &Stone) -> Vec<Stone> {
    let mut ret = Vec::with_capacity(2);
    let digits = format!("{}", s).len();
    match *s {
        val if val == 0 => ret.push(1),
        val if digits % 2 == 0 => {
            ret.push(val / 10_u64.pow(digits as u32/2));
            ret.push(val % 10_u64.pow(digits as u32/2));
        },
        val => ret.push(val*2024),
    };
    // println!("{:?} --> {:?}", s, ret);
    ret
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_smallest() {
    let input = "0";
    let mut o = parse(input).unwrap();
    assert_eq!(blink(1, &mut o), 1);
    assert_eq!(blink(1, &mut o), 1);
    assert_eq!(blink(1, &mut o), 2);
    assert_eq!(blink(1, &mut o), 4);
}

#[test]
fn test_small() {
    let input = "0 0";
    let mut o = parse(input).unwrap();
    println!("0: {:?}", o);
    blink(1, &mut o);
    println!("1: {:?}", o);
    assert_eq!(sum(&o), 2);
}

#[test]
fn test_mid() {
    let input = "0 1 10 99 999";
    let mut o = parse(input).unwrap();
    println!("0: {:?}", o);
    blink(1, &mut o);
    println!("1: {:?}", o);
    assert_eq!(sum(&o), 7);
    // assert_eq!(to_string(&o), "1 2024 1 0 9 9 2021976");
}

#[test]
fn test() {
    let input = "125 17";
    let o = parse(input).unwrap();
    
    // blink(1, o);
    // assert_eq!(str(&o), "253000 1 7");
    assert_eq!(blink(1, &mut o.clone()), 3);
    // o = blink(1, o);
    // assert_eq!(str(&o), "253 0 2024 14168");
    assert_eq!(blink(2, &mut o.clone()), 4);
    // o = blink(1, o);
    // assert_eq!(str(&o), "512072 1 20 24 28676032");
    assert_eq!(blink(3, &mut o.clone()), 5);
    // o = blink(1, o);
    // assert_eq!(str(&o), "512 72 2024 2 0 2 4 2867 6032");
    assert_eq!(blink(4, &mut o.clone()), 9);
    // o = blink(1, o);
    // assert_eq!(str(&o), "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
    assert_eq!(blink(5, &mut o.clone()), 13);
    // o = blink(1, o);
    // assert_eq!(str(&o), "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2");
    assert_eq!(blink(6, &mut o.clone()), 22);
    assert_eq!(blink(25, &mut o.clone()), 55312);
    assert!(blink(75, &mut o.clone()) > 3010176871);
}

#[test]
fn test_1() {
    let input = "1750884 193 866395 7 1158 31 35216 0";
    let mut o = parse(input).unwrap();
    assert_eq!(blink(25, &mut o), 231278);
}

}
