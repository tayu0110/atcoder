use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    println!("{}", a.iter().position(|v| *v == 1).unwrap() + 1)
}
