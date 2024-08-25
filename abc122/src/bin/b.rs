#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {s: Chars}

    let mut res = 0;
    let atcg = ['A', 'T', 'C', 'G'];
    for i in 0..s.len() {
        let ns = (i..s.len()).take_while(|i| atcg.iter().any(|nc| *nc == s[*i])).map(|i| s[i]).collect::<String>();
        res = std::cmp::max(res, ns.len());
    }

    println!("{}", res);
}
