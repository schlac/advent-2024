fn main() {
    let input = include_str!("../input.txt");
    let r = &parse(input);
    println!("{}", solve1(r));
    println!("{}", solve2(r));
}

type Reports = Vec<Vec<i8>>;

fn parse(input: &str) -> Reports {
    input
        .lines()
        .fold(vec![], |mut v, line| {
            v.push(line.split_whitespace().map(|s|
                s.parse().expect("nan")).collect::<Vec<i8>>()
            );
            v
        })
}

fn solve(r: Reports) -> impl Iterator<Item=Vec<i8>> {
    r.into_iter().filter(|r| r.iter().fold((true, 0i8, 0i8), |(ok, d, p), c| {
        let c = *c;
        if !ok || p == 0 { return (ok, d, c); }
        // println!("{} {} {}", ok, d, p);
        let diff = p - c;
        match diff.abs() {
            ..1 => (false, d, c),
            1..=3 => (diff * d >= 0, diff, c),
            _ => (false, d, c),
        }
    }).0)
}

fn solve1(r: &Reports) -> usize {
    solve(r.clone()).count()
}

fn solve2(r: &Reports) -> usize {
    let mut n = 0;
    for r in r {
        let mut rv = Vec::with_capacity(r.len());
        for i in 0..r.len() {
            let mut rm = r.clone();
            rm.remove(i);
            rv.push(rm);
        }
        if solve(rv).next().is_some() {
            n += 1;
        }
    }
    n
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    let r = &parse(input);
    assert_eq!(solve1(r), 2);
    assert_eq!(solve2(r), 4);
}

}
