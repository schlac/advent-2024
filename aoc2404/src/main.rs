use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let c = count(&input);
    println!("{c}");
}

pub fn count(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test1f() {
    let input = r"XMAS";
    assert_eq!(count(input), 1);
}

#[test]
fn test1b() {
    let input = r"SAMX";
    assert_eq!(count(input), 1);
}

#[test]
fn test1d() {
    let input = r"X\nM\nA\nS";
    assert_eq!(count(input), 1);
}

#[test]
fn test1u() {
    let input = r"S\nA\nM\nX";
    assert_eq!(count(input), 1);
}

#[test]
fn test1ud() {
    let input = r"
X\n.\n.\nS
.\nM\nA\n.
.\nM\nA\n.
X\n.\n.\nS
";
    assert_eq!(count(input), 2);
}

#[test]
fn test1du() {
    let input = r"
S\n.\n.\nX
.\nA\nM\n.
.\nA\nM\n.
S\n.\n.\nX
";
    assert_eq!(count(input), 2);
}

#[test]
fn test() {
    let input = r"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
    ";
    assert_eq!(count(input), 18);
}

}
