use proconio::*;

fn main() {
    input! {a: usize, b: usize, c: usize}
    println!("{}", c / a.min(b))
}
