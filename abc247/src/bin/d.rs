#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {q: usize}

    let mut nt = std::collections::VecDeque::new();
    for _ in 0..q {
        input! {t: usize}

        if t == 1 {
            input! {x: usize, c: usize}
            nt.push_back((x, c));
        } else {
            input! {mut c: usize}
            let mut res = 0;
            while let Some((x, nc)) = nt.pop_front() {
                if nc <= c {
                    c -= nc;
                    res += nc * x;
                } else {
                    res += c * x;
                    nt.push_front((x, nc - c));
                    break;
                }
            }
            println!("{}", res);
        }
    }
}
