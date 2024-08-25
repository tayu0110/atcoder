use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut s: marker::Chars, c: [usize; n]}

    let mut t = vec![vec![]; m];
    for (i, c) in c.into_iter().enumerate() {
        t[c - 1].push(i);
    }

    for v in t {
        if v.is_empty() {
            continue;
        }

        let len = v.len();
        let mut prev = 'a';
        for i in 0..len {
            if i == 0 {
                prev = s[v[i]];
                s[v[i]] = s[v[len - 1]];
            } else {
                std::mem::swap(&mut s[v[i]], &mut prev);
            }
        }
    }

    println!("{}", s.iter().join(""))
}
