#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {h: usize, w: usize, mut c: [[usize; 10]; 10], a: [[i32; w]; h]}

    for k in 0..10 {
        for i in 0..10 {
            for j in 0..10 {
                c[i][j] = std::cmp::min(c[i][j], c[i][k] + c[k][j]);
            }
        }
    }

    let res = a.iter().flatten().filter(|v| **v != -1).fold(0, |s, v| s + c[*v as usize][1]);
    println!("{}", res);
}
