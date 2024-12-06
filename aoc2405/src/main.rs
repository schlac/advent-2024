use std::fs;
use std::cmp;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let mut man = parse(&input).unwrap();
    let v = valid(&man);
    let i = invalid(&mut man);
    println!("{v} / {i}");
}

type Rules = Vec<[u32;2]>;

trait Pages {
    fn middle(&self) -> u32;
    fn is_ordered(&self, rules: &Rules) -> bool;
    fn order(&mut self, rules: &Rules) -> &Self;
}

struct Man {
    rules: Vec<[u32;2]>,
    pages: Vec<Vec<u32>>,
}

impl Pages for Vec<u32> {
    fn middle(&self) -> u32 {
        if self.len() > 2 {
            self[self.len()/2]
        } else {
            0
        }
    }
    fn is_ordered(&self, rules: &Rules) -> bool {
        for r in rules.iter() {
            let mut found1 = false;
            let mut found2 = false;
            for p in self.iter() {
                if *p == r[0] {
                    if found2 { return false; }
                    found1 = true;
                }
                else if *p == r[1] {
                    if found1 { continue; }
                    found2 = true;
                }
            }
        }
        true
    }
    fn order(&mut self, rules: &Rules) -> &Vec<u32> {
        // println!("u {self:?}");
        self.sort_by(|a, b| {
            for r in rules {
                let l = r[0];
                let h = r[1];
                if *a == l && *b == h {
                    return cmp::Ordering::Less;
                }
                if *a == h && *b == l {
                    return cmp::Ordering::Greater;
                }
            }
            cmp::Ordering::Equal
        });
        // println!("o {self:?}");
        self
    }
}

impl Man {
    fn new() -> Self {
        Man {
            rules: vec![],
            pages: vec![],
        }
    }
}

fn parse(input: &str) -> Option<Man> {
    let mut m = Man::new();

    for line in input.lines() {
        match line
            .trim_start()
            .trim_end()
            .split_once('|')
        {
            Some((x,y)) => {
                // println!("= {x} / {y}");
                m.rules.push([
                    x.parse::<u32>().unwrap(),
                    y.parse::<u32>().unwrap(),
                ]);
            },
            _ => {
                let st = line
                    .trim_start().trim_end()
                    .split(',').filter(|s|!s.is_empty());
                let mut v: Vec<u32> = vec![];
                for s in st {
                    v.push(s.parse::<u32>().unwrap());
                }

                    // println!("{line} {v:?}");
                    if !v.is_empty() {
                    m.pages.push(v);
                }
            },
        }
    }
    Some(m)
}

fn valid(m: &Man) -> i64 {
    m.pages.iter().filter(|p| p.is_ordered(&m.rules))
        .fold(0, |s,p| s + i64::from(p.middle()))
}

fn invalid(m: &mut Man) -> i64 {
    m.pages.iter_mut().filter(|p| !p.is_ordered(&m.rules))
        .map(|p| p.order(&m.rules))
        .fold(0, |s,p| s + i64::from(p.middle()))
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_0() {
    let input = "
2|1

1,2,3";
    let mut m = parse(input).unwrap();
    assert_eq!(valid(&m), 0, "valid");
    assert_eq!(invalid(&mut m), 1, "invalid");
}

#[test]
fn test_2() {
    let input = "
1|2

1,2,3";
    let mut m = parse(input).unwrap();
    assert_eq!(valid(&m), 2, "valid");
    assert_eq!(invalid(&mut m), 0, "invalid");
}

#[test]
fn test_orders() {
    let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    let mut m = parse(input).unwrap();
    assert_eq!(valid(&m), 143, "valid");
    assert_eq!(invalid(&mut m), 123, "invalid");
}

}
