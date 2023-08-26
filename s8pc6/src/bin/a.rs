use proconio::*;

fn main() {
    input! {n: usize, t: u32, a: [u32; n-1]}
    let sum = a.iter().sum::<u32>();
    println!("{}", (sum + t - 1) / t)
}
