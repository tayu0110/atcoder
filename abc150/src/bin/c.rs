#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, p: [usize; n], q: [usize; n]}

    let (mut np, mut nq) = (0, 0);
    for (i, v) in (1..=n).permutations(n).enumerate() {
        if v == p {
            np = i;
        }
        if v == q {
            nq = i;
        }
    }

    println!("{}", std::cmp::max(np, nq) - std::cmp::min(np, nq));
}
