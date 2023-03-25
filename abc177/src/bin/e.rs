#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn pairwise_coprime(a: &[usize]) -> bool {
    let max = *a.iter().max().unwrap();
    let mut d = vec![std::usize::MAX; max+1];
    for i in 2..=max {
        if d[i] == std::usize::MAX {
            for j in (1..=max).take_while(|j| *j * i <= max) {
                d[i*j] = i;
            }
        }
    }

    let mut set = std::collections::HashSet::new();
    for na in a {
        let mut now = *na;
        let mut tmp = std::collections::HashSet::new();
        while d[now] != std::usize::MAX {
            tmp.insert(d[now]);
            now /= d[now];
        }

        for now in tmp {
            if set.contains(&now) {
                return false;
            }
            set.insert(now);
        }
    }

    true
}

fn setwise_coprime(a: &[usize]) -> bool {
    a.into_iter().fold(0, |s, v| gcd(s, *v)) == 1
}

#[fastout]
fn main() {
    input! {n: usize, a: [usize; n]}

    if pairwise_coprime(&a) {
        println!("pairwise coprime");
    } else if setwise_coprime(&a) {
        println!("setwise coprime");
    } else {
        println!("not coprime");
    }
}
