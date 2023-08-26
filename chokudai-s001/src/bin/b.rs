use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}
    println!("{}", a.iter().sum::<usize>())
}
