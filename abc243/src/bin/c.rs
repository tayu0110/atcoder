#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(usize, usize); n], s: Chars}
    let mut p = p.into_iter().zip(0..n).collect::<Vec<((usize, usize), usize)>>();
    p.sort();

    let mut map = std::collections::HashMap::new();
    for ((_, y), i) in p {
        map.entry(y).or_insert(vec![]).push(s[i]);
    }

    for (_, v) in map {
        let mut found = false;
        for c in v {
            if c == 'R' {
                found = true;
            } else if found {
                println!("Yes");
                std::process::exit(0);
            }
        }
    }

    println!("No");
}
