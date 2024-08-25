use proconio::*;

fn main() {
    input! {n: usize, a: [i32; n - 1]}
    println!("{}", -a.into_iter().sum::<i32>())
}
