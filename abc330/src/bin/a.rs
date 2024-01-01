use proconio::*;

fn main() {
    input! {n: usize, l: usize, a: [usize; n]}
    println!("{}", a.into_iter().filter(|&a| a >= l).count())
}
