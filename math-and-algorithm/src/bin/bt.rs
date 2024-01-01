use math::miller_rabin_test;
use proconio::*;

fn main() {
    input! {l: u64, r: u64}

    println!("{}", (l..=r).filter(|&i| miller_rabin_test(i)).count())
}
