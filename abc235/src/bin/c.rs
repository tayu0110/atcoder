#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, q: usize, a: [usize; n], p: [(usize, usize); q]}

    let mut map = std::collections::HashMap::new();
    for (i, v) in a.iter().enumerate() {
        map.entry(*v).or_insert(vec![]).push(i);
    }

    for (x, k) in p {
        if let Some(v) = map.get(&x) {
            if v.len() >= k {
                println!("{}", v[k-1]+1);
            } else {
                println!("-1");
            }
        } else {
            println!("-1");
        }
    }
}
