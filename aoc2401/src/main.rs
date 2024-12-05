use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let c = dist(parse(&input).unwrap());
    let m = sim(parse(&input).unwrap());
    println!("{c} / {m}");
}

fn parse(input: &str) -> Option<(Vec<i32>, Vec<i32>)> {
    let mut l: Vec<i32> = vec![];
    let mut r: Vec<i32> = vec![];

    for line in input.lines() {
        match line
            .trim_start()
            .trim_end()
            .split_once(' ')
        {
            Some((x,y)) => {
                // println!("= {x} / {y}");
                l.push(x.parse::<i32>().unwrap());
                r.push(y.trim_start().parse::<i32>().unwrap());
            },
            _ => {
                // println!("x {line}");
                continue;
            },
        }
    }
    Some((l, r))
}

fn dist(lr: (Vec<i32>, Vec<i32>)) -> i64 {
    let mut l = lr.0;
    let mut r = lr.1;
    l.sort();
    r.sort();
    l.iter().zip(r).fold(0, |s,x| s + i64::from((x.0 - x.1).abs()))
}

fn sim(lr: (Vec<i32>, Vec<i32>)) -> i64 {
    let l = lr.0;
    let r = lr.1;
    let mut rc = HashMap::new();
    for rv in r.iter() {
        rc.entry(rv).and_modify(|rc| *rc += 1).or_insert(1);
    }
    // for (k,v) in rc.iter() {
    //     println!("has {k}:{v}");
    // }
    l.iter().fold(0, |s,lv| s + (i64::from(*lv) * rc.get(lv).unwrap_or(&0)))
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test1() {
    let input = "
3 2
4 4
";
    assert_eq!(dist(parse(input).unwrap()), 1);
}

#[test]
fn test_dist() {
    let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(dist(parse(input).unwrap()), 11);
}

#[test]
fn test_sim() {
    let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(sim(parse(input).unwrap()), 31);
}

}
