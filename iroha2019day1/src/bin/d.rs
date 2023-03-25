#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, mut x: usize, mut y: usize, mut a: [usize; n]};
    a.sort();
    let [x, y] = a.into_iter().rev().enumerate().fold([x, y], |mut r, (i, v)| { r[i%2] += v; r });

    if x > y {
        println!("Takahashi");
    } else if x == y {
        println!("Draw");
    } else {
        println!("Aoki");
    }
}
