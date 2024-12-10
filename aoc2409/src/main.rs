use std::collections::VecDeque;
use std::fs;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let o = parse(&input).unwrap();
    let r1 = checksum(defrag(o.clone()));
    let r2 = 0;
    println!("{r1} / {r2}");
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Block {
    id: usize,
}

impl Block {
    fn is_free(&self) -> bool {
        self.id == usize::MAX
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct File {
    id: usize,
    pos: usize,
    len: usize,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "'{{{}:{}-{}}}'", self.id, self.pos, self.len)
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct Obj {
    files: Vec<File>,
    blocks: VecDeque<Block>,
}

impl std::fmt::Display for Obj {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "O''")
    }
}

impl Obj {
    fn new() -> Self {
        Obj {
            files: vec![],
            blocks: VecDeque::with_capacity(1000),
        }
    }
    fn blocks(&self) -> String {
        let len = self.files.iter().last().map(|f| f.pos + f.len).unwrap();
        let mut str = ".".repeat(len).to_string();
        let _ = self.files.iter().inspect(
            |s| str.replace_range(
                s.pos..s.pos+s.len,
                &format!("{}", s.id).repeat(s.len)
            )).collect::<Vec<&File>>();
        str
    }
    fn free(&self) -> usize {
        self.blocks.iter().filter(|x|x.is_free()).count()
    }
    fn used(&self) -> usize {
        self.files.iter().fold(0, |s,f| s + f.len)
    }
    fn print(&self) {
        println!("{}", self.str());
    }
    fn str(&self) -> String {
        let mut s = "".to_owned();
        for b in self.blocks.iter() {
            if b.is_free() {
                s.push('.');
            } else {
                s.push_str(&format!("{}", b.id));
            }
        }
        s
    }
}

fn parse(input: &str) -> Option<Obj> {
    let mut o = Obj::new();

    let mut id: usize = 0;
    let mut pos: usize = 0;
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        if !c.is_ascii_digit() { continue; }
        let len = usize::try_from(c.to_digit(10)?).unwrap();
        for _ in 0..len {
            o.blocks.push_back(Block{ id });
        }
        let c2 = chars.next();
        o.files.push(File { id, pos, len });
        println!("{c} => f{id}:{pos}-{len}");
        if c2 != None {
            if !c2?.is_ascii_digit() { continue; }
            let gap_len = usize::try_from(c2?.to_digit(10)?).unwrap();
            for _ in 0..gap_len {
                o.blocks.push_back(Block{ id: usize::MAX });
            }
            id += 1;
            pos += len + gap_len;
        }
    }
    o.blocks.make_contiguous();
    Some(o)
}

fn defrag(mut o: Obj) -> Obj {
    o.print();
    let used = o.used();
    let len = o.blocks.len();
    for i in 0..len {
        if !o.blocks.get(i).expect("bang").is_free() {
            continue;
        }
        if i >= used {
            break;
        }
        for j in 1..len+1 {
            let jj = len-j;
            if o.blocks.get(jj).expect("boom").is_free() {
                continue;
            }
            println!("swap: {i}:{}", jj);
            o.blocks.swap(i, jj);
            break;
        }
    }
    o.print();
    o
}

fn checksum(o: Obj) -> usize {
    o.blocks.iter().enumerate()
        .filter(|x|!x.1.is_free())
        .fold(0, |s,x| s + x.0*x.1.id)
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_small() {
    let input = "
12345
";
    let o = parse(input).unwrap();
    assert_eq!(o.blocks(), "0..111....22222");
    assert_eq!(defrag(o).str(), "022111222......");
}

#[test]
fn test_mid() {
    let input = "
2333133121414131402
";
    let o = parse(input).unwrap();
    assert_eq!(o.blocks(), "00...111...2...333.44.5555.6666.777.888899");
    assert_eq!(checksum(defrag(o)), 1928);
}

}
