use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, x: usize, a: [usize; n]};

    let v = a.into_iter().filter(|v| *v != x).collect_vec();
    for i in 0..v.len() {
        if i > 0 {
            print!(" ");
        }
        print!("{}", v[i]);
    }
    println!("");
}
