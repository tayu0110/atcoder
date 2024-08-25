use math::MathInt;
use proconio::*;

fn solve(n: u64, m: u64, k: u64) -> u64 {
    let min = [n, m, k].into_iter().min().unwrap();
    let res = if min == n {
        let (m, k) = (m - n, k - n);
        if m == 1 && k == 0 {
            0
        } else {
            1
        }
    } else {
        let (n, m) = (n - k, m - k);
        if m == 1 {
            0
        } else {
            let n = n % m;
            2u64.pow_mod(n as u64, 10)
        }
    };

    (res * 2u64.pow_mod(min as u64, 10)) % 10
}

fn main() {
    input! {t: usize, query: [(u64, u64, u64); t]}

    for (n, m, k) in query {
        println!("{}", solve(n, m, k));
    }
}
