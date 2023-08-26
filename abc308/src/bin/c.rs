use itertools::Itertools;
use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut p = p.into_iter().enumerate().collect::<Vec<_>>();
    p.sort_by(|&l, &r| {
        let (a, b) = l.1;
        let (s, t) = r.1;

        let res = (a * (s + t)).cmp(&(s * (a + b)));
        match res {
            std::cmp::Ordering::Equal => r.0.cmp(&l.0),
            _ => res,
        }
    });

    println!("{}", p.into_iter().rev().map(|(res, _)| res + 1).join(" "))
}
