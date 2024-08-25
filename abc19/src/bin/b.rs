use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Chars}

    let mut t = vec![];
    for c in s {
        match t.last_mut() {
            Some((pc, cnt)) if *pc == c => *cnt += 1,
            _ => t.push((c, 1)),
        }
    }

    println!("{}", t.iter().map(|(c, cnt)| format!("{c}{cnt}")).join(""))
}
