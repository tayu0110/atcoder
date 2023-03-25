#[allow(unused_imports)]
use proconio::{input, marker::{Chars, Bytes}, source::line::LineSource};
#[allow(unused_imports)]
use itertools::Itertools;

fn main() {
    input! {n: usize, a: [usize; n]}

    let mut e = a.iter().filter(|v| **v % 2 == 0).collect_vec();
    let mut o = a.iter().filter(|v| **v % 2 == 1).collect_vec();
    e.sort();
    o.sort();

    if e.len() <= 1 && o.len() <= 1 {
        println!("-1");
    } else {
        let er = if e.len() > 1 {
            e[e.len()-1] + e[e.len()-2]
        } else {
            0
        };
        let or = if o.len() > 1 {
            o[o.len()-1] + o[o.len()-2]
        } else {
            0
        };
        println!("{}", std::cmp::max(or, er));
    }
}
