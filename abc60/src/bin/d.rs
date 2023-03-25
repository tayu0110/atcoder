#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, w: usize, p: [(usize, usize); n]}

    let mut map = std::collections::BTreeMap::new();
    map.insert(0, 0);
    for (nw, nv) in p {
        let mut next = std::collections::BTreeMap::new();
        for (k, v) in map.iter() {
            *next.entry(*k).or_insert(0) = std::cmp::max(*next.entry(*k).or_insert(0), *v);
            if *k + nw <= w {
                *next.entry(*k + nw).or_insert(0) = std::cmp::max(*next.entry(*k + nw).or_insert(0), *v + nv);
            }
        }
        std::mem::swap(&mut map, &mut next);
    }

    let res = map.values().max().unwrap();
    println!("{}", res);
}
