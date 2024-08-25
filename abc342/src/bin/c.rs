use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, mut s: marker::Chars, q: usize}

    let mut col = [0; 256];
    for c in b'a'..=b'z' {
        col[c as usize] = c;
    }

    for _ in 0..q {
        input! {c: char, d: char}

        let (c, d) = (c as u8, d as u8);
        for t in b'a'..=b'z' {
            if col[t as usize] == c {
                col[t as usize] = d;
            }
        }
    }

    for i in 0..n {
        s[i] = col[s[i] as usize] as char;
    }

    println!("{}", s.iter().join(""))
}
