use proconio::*;

fn main() {
    input! {n: usize, x: usize, a: [usize; n]}
    println!("{}", if a.contains(&x) { "Yes" } else { "No" })
}
