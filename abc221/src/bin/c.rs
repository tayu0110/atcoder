#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};
use itertools::*;

fn main() {
    input! {n: Chars}

    let mut res = 0;
    for v in (0..n.len()).permutations(n.len()) {
        let n = v.iter().fold(vec![], |mut v, i| { v.push(n[*i]); v });
        for i in 1..n.len() {
            if n[i] == '0' {
                continue;
            }
            let s = n[0..i].iter().collect::<String>().parse::<usize>().unwrap();
            let t = n[i..].iter().collect::<String>().parse::<usize>().unwrap();
    
            res = std::cmp::max(res, s*t);
        }
    }

    println!("{}", res);
}
