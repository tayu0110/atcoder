use itertools::Itertools;
use proconio::*;

fn main() {
    input! {q: usize}

    let mut keep = vec![];
    let mut buf = vec![];
    let mut res = vec![];
    for _ in 0..q {
        input! {ty: u8}
        if ty == 1 {
            input! {c: char}
            buf.push(c as u8);
        } else if ty == 2 {
            keep.extend(buf.drain(..).rev());
        } else {
            res.push(
                buf.drain(..)
                    .chain(keep.drain(..).rev())
                    .map(|c| c as char)
                    .collect::<String>(),
            );
        }
    }

    let w = buf.len() + 1;
    res.push(
        buf.drain(..)
            .chain(keep.drain(..).rev())
            .map(|c| c as char)
            .collect::<String>(),
    );
    println!("{} {w}", res.len());
    println!("{}", res.iter().map(|r| format!("# {r}")).join("\n"))
}
