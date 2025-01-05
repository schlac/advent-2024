use partial_application::partial;

fn main() {
    let input = include_str!("../input.txt");
    let p = &parse(input);
    println!("{}", solve1(p));
    println!("{}", solve2(p));
}

type Base = u64;
type Calc = Vec<Base>;
type Calcs = Vec<Calc>;

type Operation = fn(a: &Base, b: &Base) -> Base;

const OPERATIONS1: [Operation; 2] = [
    sum,
    product,
];

const OPERATIONS2: [Operation; 3] = [
    sum,
    product,
    concat,
];

fn sum(a: &Base, b: &Base) -> Base {
    a + b
}

fn product(a: &Base, b: &Base) -> Base {
    a * b
}

fn concat(a: &Base, b: &Base) -> Base {
    (a.to_string() + &b.to_string()).parse::<Base>().expect("nan")
}

fn parse(input: &str) -> Calcs {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .fold(vec![], |mut v, line| {
            v.push(line.split_whitespace().map(|s|
                s.trim_end_matches(':').parse().expect("nan")).collect::<Vec<Base>>()
            );
            v
        })
}

fn is_solvable(operations: &[Operation], v: &Vec<Base>) -> bool {
    let mut iter = v.iter();
    let target = *iter.next().expect("no values");
    let first = *iter.next().expect("too few elements");
    let calc = iter.fold(vec![first], |state, c| {
        let mut new_state = Vec::with_capacity(state.len() * operations.len());
        for val in state {
            for f in operations {
                let new_val = f(&val, &c);
                if new_val <= target {
                    new_state.push(new_val);
                }
            }
        }
        new_state
    });
    let r = calc.contains(&target);
    // println!("{} {:?} {} {:?}", target, calc, r, v);
    r
}

fn solve1(calcs: &Calcs) -> usize {
    calcs.clone().into_iter()
        .filter(partial!(is_solvable, &OPERATIONS1, _))
        .map(|c| c[0]).sum::<Base>() as usize
}

fn solve2(calcs: &Calcs) -> usize {
    calcs.clone().into_iter()
        .filter(partial!(is_solvable, &OPERATIONS2, _))
        .map(|c| c[0]).sum::<Base>() as usize
}

#[cfg(test)]
mod tests {
use super::*;

    #[test]
    fn test_small_concat() {
        let input = "
190: 1 90
";
    let r = &parse(input);
    assert_eq!(solve2(r), 190);
}

    #[test]
    fn test() {
        let input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    let r = &parse(input);
    assert_eq!(solve1(r), 3749);
    assert_eq!(solve2(r), 11387);
}

}
