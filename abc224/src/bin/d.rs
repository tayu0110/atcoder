#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn encode(v: &[usize]) -> usize {
    v.iter().fold(0, |s, v| s * 10 + *v)
}

fn decode(mut v: usize) -> Vec<usize> {
    let mut buf = vec![];
    while v > 0 {
        buf.push(v % 10);
        v /= 10;
    }

    while buf.len() != 10 {
        buf.push(0);
    }
    buf.reverse();

    buf
}

fn main() {
    const N: usize = 10;
    input! {m: usize, e: [(usize, usize); m], p: [usize; 8]};

    let mut s = vec![0; N];
    for (i, v) in p.into_iter().enumerate() {
        s[v] = i+1;
    }
    
    let mut t= vec![vec![]; N];
    for (u, v) in e {
        t[u].push(v);
        t[v].push(u);
    }

    let mut nt = std::collections::VecDeque::new();
    nt.push_back((encode(&s), 0));
    let mut memo = std::collections::HashMap::new();

    while let Some((v, nd)) = nt.pop_front() {
        if memo.contains_key(&v) {
            continue;
        }
        memo.insert(v, nd);

        let mut v = decode(v);

        for i in 0..N {
            if v[i] == 0 {
                for &to in &t[i] {
                    v.swap(i, to);
                    let w = encode(&v);

                    if !memo.contains_key(&w) {
                        nt.push_back((w, nd+1));
                    }

                    v.swap(i, to);
                }
            }
        }
    }

    if let Some(res) = memo.get(&123456780) {
        println!("{}", res);
    } else {
        println!("-1");
    }
}
