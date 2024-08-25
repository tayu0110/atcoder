#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    source::line::LineSource,
};

fn main() {
    input! {n: usize, mut a: [usize; n]}
    let mut map = std::collections::HashMap::new();
    for v in a {
        *map.entry(v).or_insert(0) += 1;
    }

    let mut a = vec![];
    for (k, v) in map {
        a.push(k);
        if v > 1 {
            a.resize(a.len() + v - 1, usize::MAX);
        }
    }

    a.sort();

    let mut nt = a.into_iter().collect::<std::collections::VecDeque<_>>();

    let mut req = 1;

    while !nt.is_empty() {
        if req == *nt.front().unwrap() {
            nt.pop_front().unwrap();
            req += 1;
        } else {
            if nt.len() < 2 {
                break;
            }

            nt.pop_back().unwrap();
            nt.pop_back().unwrap();

            req += 1;
        }
    }

    println!("{}", req - 1);
}
