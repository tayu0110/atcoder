use std::collections::VecDeque;

use itertools::Itertools;
use proconio::*;

fn main() {
    input! {s: marker::Bytes}

    let mut s = s
        .into_iter()
        .dedup_with_count()
        .map(|c| c.1)
        .collect::<VecDeque<_>>();

    if s.len() == 1 {
        println!("{}", s.pop_front().unwrap() as char);
        return;
    }

    for i in 0.. {
        let c = if i % 2 == 0 { b'o' } else { b'x' };
        if s.front().unwrap() != &c {
            s.pop_front();
        } else if s.back().unwrap() != &c {
            s.pop_back();
        }

        if s.len() == 1 {
            println!("{}", c as char);
            return;
        }
    }
}
