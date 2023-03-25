#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut nt = std::collections::BinaryHeap::new();
    for i in 0..=m {
        nt.push(std::cmp::Reverse(i));
    }

    let mut map = std::collections::HashMap::new();
    for &a in a.iter().take(m-1) {
        *map.entry(a).or_insert(0) += 1;
    }

    let mut res = std::usize::MAX;
    for i in m-1..n {
        *map.entry(a[i]).or_insert(0) += 1;

        while let Some(std::cmp::Reverse(now)) = nt.pop() {
            if !map.contains_key(&now) {
                nt.push(std::cmp::Reverse(now));
                break;
            }
        }

        if i >= m {
            *map.entry(a[i-m]).or_insert(0) -= 1;
            if *map.get(&a[i-m]).unwrap() == 0 {
                map.remove(&a[i-m]);
                nt.push(std::cmp::Reverse(a[i-m]));
            }
        }

        if let Some(&std::cmp::Reverse(r)) = nt.iter().next() {
            res = std::cmp::min(res, r);
        }
    }

    println!("{}", res);
}
