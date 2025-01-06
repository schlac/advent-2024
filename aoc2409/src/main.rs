use std::fs;
use Block::*;

fn main() {
    let file_path = "./input.txt";
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file {file_path}");
    let o = Obj::parse(&input).unwrap();
    let r1 = defrag(o.clone()).checksum();
    let r2 = defrag2(o.clone()).checksum();
    println!("{r1} / {r2}");
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
enum Block {
    Free,
    Used(usize),
}

impl Block {
    fn is_free(&self) -> bool {
        match self {
            Free => true,
            Used(_) => false,
        }
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
enum Blocks {
    Blocks(Vec<Block>),
}

impl Blocks {
    fn unwrap(&self) -> Vec<Block> {
        match self {
            Blocks::Blocks(b) => b.to_vec(),
        }
    }
    fn checksum(&self) -> usize {
        self.unwrap().iter().enumerate()
            .filter(|x|!x.1.is_free())
            .fold(0, |s,(n,b)| s + n * match b { Free => &0usize, Used(id) => id, })
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
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
    len: usize,
}

#[inline(always)]
fn write_block(b: &Block, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    match b {
        Free => write!(fmt, "."),
        Used(id) => write!(fmt, "{}", id),
    }
}

impl std::fmt::Display for Blocks {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for b in self.unwrap().iter() {
            write_block(b, fmt)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for Obj {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for b in self.blocks().iter() {
            write_block(b, fmt)?;
        }
        Ok(())
    }
}

impl Obj {
    fn parse(input: &str) -> Option<Obj> {
        let mut files = vec![];
        let mut id: usize = 0;
        let mut pos: usize = 0;
        let mut chars = input.chars();
        while let Some(c) = chars.next() {
            if !c.is_ascii_digit() { continue; }
            let len = usize::try_from(c.to_digit(10)?).ok()?;
            let c2 = chars.next();
            files.push(File { id, pos, len });
            // println!("{c} => f{id}:{pos}..={}", pos + len - 1);
            pos += len;
            id += 1;
            if c2 != None {
                if !c2?.is_ascii_digit() { continue; }
                let gap_len = usize::try_from(c2?.to_digit(10)?).ok()?;
                pos += gap_len;
            }
        }
        let last = files.iter().last()?;
        let len = last.pos + last.len;
        Some(Obj{ files, len, })
    }
    fn blocks(&self) -> Vec<Block> {
        let mut blocks = self.files.iter().fold(
            Vec::with_capacity(self.len),
            |mut blocks, c| {
                for _ in blocks.len()..c.pos {
                    blocks.push(Free);
                }
                for _ in 0..c.len {
                    blocks.push(Used(c.id));
                }
                blocks
            }
        );
        for _ in blocks.len()..self.len {
            blocks.push(Free);
        }
        blocks
    }
    fn used(&self) -> usize {
        self.files.iter().fold(0, |s,f| s + f.len)
    }
}

fn defrag(o: Obj) -> Blocks {
    // o.print();
    let used = o.used();
    let len = o.len;
    let mut blocks = o.blocks();
    for i in 0..len {
        match blocks.get(i).unwrap() {
            Used(_) => continue,
            Free => {
                if i >= used {
                    break;
                }
                for j in 1..len+1 {
                    let jj = len-j;
                    match blocks.get(jj).unwrap() {
                        Free => continue,
                        Used(_) => {
                            // println!("swap: {i}:{}", jj);
                            blocks.swap(i, jj);
                            break;
                        }
                    }
                }
            }
        }
    }
    Blocks::Blocks(blocks)
}

fn defrag2(mut o: Obj) -> Blocks {
    // o.print();
    let mut files = o.files.clone();
    files.reverse();
    for mut f in files {
        let len = f.len;
        // println!("\n{:?}", o.files);
        let mut n = 1;
        while n < o.files.len() {
            let pfile = o.files.get(n-1).unwrap();
            let nfile = o.files.get(n).unwrap();
            n += 1;

            let pfile_end = pfile.pos + pfile.len;
            if pfile_end > f.pos {
                break;
            }
            let gap_len = nfile.pos.checked_sub(pfile_end);
            if let Some(gap_len) = gap_len {
                if gap_len >= len {
                    let r = o.files.binary_search_by(|p| p.pos.cmp(&f.pos)).unwrap();
                    o.files.remove(r);
                    f.pos = pfile_end;
                    o.files.insert(n-1, f);
                }
            }
        }
    }
    Blocks::Blocks(o.blocks())
}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn test_small() {
    let input = "
12345
";
    let o = Obj::parse(input).unwrap();
    assert_eq!(o.to_string(), "0..111....22222");
    assert_eq!(defrag(o.clone()).to_string(), "022111222......");
    assert_eq!(defrag2(o.clone()).to_string(), "0..111....22222");
}

#[test]
fn test_small_rev() {
    let input = "
54321
";
    let o = Obj::parse(input).unwrap();
    assert_eq!(o.to_string(), "00000....111..2");
    assert_eq!(defrag(o.clone()).to_string(), "000002111......");
    assert_eq!(defrag2(o.clone()).to_string(), "000002111......");
}

#[test]
fn test_mid() {
    let input = "
2333133121414131402
";
    let o = Obj::parse(input).unwrap();
    assert_eq!(o.to_string(), "00...111...2...333.44.5555.6666.777.888899");
    assert_eq!(defrag(o.clone()).checksum(), 1928);
    assert_eq!(defrag2(o.clone()).checksum(), 2858);
}

}
