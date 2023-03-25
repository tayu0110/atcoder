#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, k: usize, p: [usize; n]}

    let mut nt = p.iter().take(k).cloned().map(|f| std::cmp::Reverse(f)).collect::<std::collections::BinaryHeap<std::cmp::Reverse<_>>>();
    println!("{}", p.iter().take(k).min().unwrap());
    for i in k..n {
        let std::cmp::Reverse(f) = nt.pop().unwrap();
        nt.push(std::cmp::Reverse(std::cmp::max(p[i], f)));

        let std::cmp::Reverse(f) = nt.pop().unwrap();
        println!("{}", f);
        nt.push(std::cmp::Reverse(f));
    }
}
