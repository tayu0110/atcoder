use num::{BigUint, One};
use proconio::*;

fn main() {
    input! {n: usize, d: [usize; n - 1]}

    let sum = d.iter().sum::<usize>();
    let m = sum / 2;
    #[target_feature(enable = "avx2")]
    unsafe fn solve(m: usize, d: Vec<usize>) -> BigUint {
        let mut memo = BigUint::one();
        let mask = (BigUint::one() << (m + 1)) - BigUint::one();
        for d in d {
            memo |= (memo.clone() << d) & &mask;
        }
        memo
    }

    let memo = unsafe { solve(m, d) };
    println!(
        "{}",
        sum - (0..m + 1).rfind(|&i| memo.bit(i as u64)).unwrap() * 2
    )
}
