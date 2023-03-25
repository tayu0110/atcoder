#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, q: usize, p: [(usize, usize); n-1], q: [(usize, usize); q]};

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
    }

    let mut res = vec![0; n];
    for (p, x) in q {
        res[p-1] += x;
    }

    let mut ck = vec![false; n];
    let mut nt = std::collections::VecDeque::new();
    nt.push_back(0);
    while let Some(now) = nt.pop_front() {
        if ck[now] {
            continue;
        }
        ck[now] = true;
        
        for to in &t[now] {
            if !ck[*to] {
                res[*to] += res[now];
                nt.push_back(*to);
            }
        }
    }

    for (i, v) in res.into_iter().enumerate() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v);
    }
    println!("");
}
