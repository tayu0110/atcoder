#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, s: [usize; n], t: [usize; m]}

    let set = t.into_iter().collect::<std::collections::HashSet<_>>();
    let res = s.into_iter().filter(|&s| set.contains(&(s % 1000))).count();
    println!("{}", res);
}
