fn main() {
    let input = include_str!("../input.txt");
    println!("{}", solve(parse(input)));
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

fn solve(r: Reports) -> usize {
    r.into_iter().filter(|r| r.iter().fold((true, 0i8, 0i8), |(ok, d, p), c| {
        let c = *c;
        if !ok || p == 0 { return (ok, d, c); }
        // println!("{} {} {}", ok, d, p);
        let diff = p - c;
        match diff.abs() {
            ..1 => return (false, d, c),
            1..=3 => return (diff * d >= 0, diff, c),
            _ => return (false, d, c),
        }
        // match d {
        //     0 => (match p - c { ..1 => false, 1..=3 => true, _ => false, }, p - c, c),
        //     ..0 => (match p - c { ..1 => false, 1..=3 => true, _ => false, }, d, c),
        //     0.. => (match c - p { ..1 => false, 1..=3 => true, _ => false, }, d, c),
        // }
    }).0).count()
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
    assert_eq!(solve(parse(input)), 2);
}

}
