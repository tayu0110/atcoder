use std::collections::BinaryHeap;

#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
	input! {n: usize, p: [(usize, usize); n-1], s: Chars};

    let mut t =vec![vec![]; n];
    for (a, b) in p {
        let (a, b) = (a-1, b-1);

        t[a].push(b);
        t[b].push(a);
    }

    let mut s = s;
    let mut res = vec![];
    let mut nt = BinaryHeap::new();
    for (i, c) in s.iter().enumerate() {
        if c == &'W' {
            nt.push(std::cmp::Reverse(i));
        }
    }

    let mut ck = vec![false; n];
    while !nt.is_empty() {
        let std::cmp::Reverse(now) = nt.pop().unwrap();
        if s[now] == 'B' {
            continue;
        }
        if ck[now] {
            continue;
        }
        ck[now] = true;
        res.push(now);
        for to in &t[now] {
            s[*to] = (s[*to] as u8 ^ 'W' as u8 ^ 'B' as u8) as char;
            if s[*to] == 'W' && !ck[*to] {
                nt.push(std::cmp::Reverse(*to));
            }
        }
    }

    if res.len() != n {
        println!("-1");
    } else {
        for v in res {
            println!("{}", v+1);
        }
    }
}
