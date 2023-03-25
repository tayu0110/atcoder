#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, q: usize}

    let mut t = vec![];
    for _ in 0..n {
        input! {l: usize, a: [usize; l]}

        t.push(a);
    }

    for _ in 0..q {
        input! {a: usize, b: usize}

        println!("{}", t[a-1][b-1]);
    }
}
