use std::fs;
use regex::Regex;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let result_p = p(&input);
    let result_pc = pc(&input);
    println!("{result_p} / {result_pc}");
}

pub fn p(input: &str) -> u64 {
    let mut sum : u64 = 0;
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    for (_,[x,y]) in re.captures_iter(input).map(|c| c.extract()) {
        let ux = x.parse::<u64>().unwrap();
        let uy = y.parse::<u64>().unwrap();
        sum += ux * uy;
    }
    sum
}

pub fn pc(input: &str) -> u64 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|(d)(o)\(\)|(d)o(n)'t\(\)").unwrap();
    let mut dont : bool = false;
    let mut sum : u64 = 0;

    for (_,[x,y]) in re.captures_iter(input).map(|c| c.extract()) {
        match y {
            "o" => { dont = false; },
            "n" => { dont = true; },
            _ => {
                if dont { continue; }
                let ux = x.parse::<u64>().unwrap();
                let uy = y.parse::<u64>().unwrap();
                sum += ux * uy;
            },
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_simple() {
        let input = "mul(2,4)".to_string();
        assert_eq!(p(&input), 8);
    }

    #[test]
    fn test_p_simple_filter() {
        let input = "mul(5,5)-mul(32,64]".to_string();
        assert_eq!(p(&input), 25);
    }

    #[test]
    fn test_p_short_test() {
        let input = "mul(2,4)+mul(5,5)mul(11,8)+mul(8,5)".to_string();
        assert_eq!(p(&input), 161);
    }

    #[test]
    fn test_p_test() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(p(&input), 161);
    }

    #[test]
    fn test_pc_test() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(pc(&input), 48);
    }
}

