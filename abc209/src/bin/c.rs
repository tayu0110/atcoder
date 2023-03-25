#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    const MOD: usize = 1000000007;
    input! {n: usize, mut a: [usize; n]}
    a.sort();

    let mut ans = 1;
    for i in 0..n {
        if a[i] < i+1 {
            ans = 0;
        }
        ans *= a[i] - i;
        ans %= MOD;
    }

    println!("{}", ans);
}
