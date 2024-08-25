use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Chars, t: marker::Chars}

    let mut now = 0;
    let mut res = vec![];
    for (i, c) in t.into_iter().enumerate() {
        if now < s.len() && s[now] == c {
            now += 1;
            res.push(i + 1);
        }
    }

    println!("{}", res.iter().join(" "))
}
