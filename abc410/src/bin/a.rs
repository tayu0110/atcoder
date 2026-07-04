use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n], k: usize}

    println!("{}", a.into_iter().filter(|&a| a >= k).count())
}
