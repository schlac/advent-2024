use std::fs;
use std::str::FromStr;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let o = parse(&input).unwrap();
    let r1 = blink(25, o.clone()).len();
    let r2 = 0;//blink(75, o.clone()).len();
    println!("{r1} / {r2}");
}

fn parse(input: &str) -> Option<Stones> {
    let mut o = Stones::new();
    for ln in input.lines() {
        if ln.is_empty() { continue; }
        for s in ln.split_whitespace() {
            o.push(Stone::from_str(&s).unwrap());
        }
    }
    Some(o)
}

type Stone = usize;
type Stones = Vec<Stone>;

fn blink(num: usize, st: Stones) -> Stones {
    let mut stones = st;
    for i in 1..=num {
        println!("Blink {i}");
        stones = stones.into_iter().flat_map(blink_rules).collect();
    }
    stones
}

fn blink_rules(st: Stone) -> Vec<Stone> {
    let mut ret = Vec::with_capacity(2);
    let n = format!("{st}").len();
    match st {
        0 => ret.push(1),
        st if n % 2 == 0 => {
            ret.push(st / 10_usize.pow(n as u32/2));
            ret.push(st % 10_usize.pow(n as u32/2));
        },
        _ => ret.push(st*2024),
    };
    // println!("{st} {} -> {ret:?}", 10_usize.pow(n as u32/2));
    ret
}

fn str(st: &Stones) -> String {
    st.iter()
        .fold(String::new(), |r,s| format!("{r} {s}"))[1..]
        .to_string()
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test() {
    let input = "125 17";
    let mut o = parse(input).unwrap();
    
    o = blink(1, o);
    assert_eq!(str(&o), "253000 1 7");
    o = blink(1, o);
    assert_eq!(str(&o), "253 0 2024 14168");
    o = blink(1, o);
    assert_eq!(str(&o), "512072 1 20 24 28676032");
    o = blink(1, o);
    assert_eq!(str(&o), "512 72 2024 2 0 2 4 2867 6032");
    o = blink(1, o);
    assert_eq!(str(&o), "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
    o = blink(1, o);
    assert_eq!(str(&o), "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2");
    for _ in 7..=25 {
        o = blink(1, o);
    }
    assert_eq!(o.len(), 55312);
}

#[test]
fn test_mid() {
    let input = "0 1 10 99 999";
    let mut o = parse(input).unwrap();
    
    o = blink(1, o);
    assert_eq!(str(&o), "1 2024 1 0 9 9 2021976");
}

}
