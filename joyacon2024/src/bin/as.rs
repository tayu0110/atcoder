use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    println!("{}", a.into_iter().map(|a| a.trailing_zeros()).sum::<u32>())
}
