use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let c = dist(&input);
    let m = 0;
    println!("{c} / {m}");
}

fn dist(input: &str) -> i64 {
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
    l.sort();
    r.sort();
    l.iter().zip(r).fold(0, |s,x| s + i64::from((x.0 - x.1).abs()))
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
    assert_eq!(dist(input), 1);
}

#[test]
fn test() {
    let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(dist(input), 11);
}

}
