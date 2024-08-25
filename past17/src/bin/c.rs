use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], p: [usize; 10]}

    println!("{}", a.into_iter().map(|a| p[a - 1]).sum::<usize>())
}
