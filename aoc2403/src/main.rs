use std::fs;
use regex::Regex;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let result = p(&input);
    println!("{result}");
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
}

