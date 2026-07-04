use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize}

    let mut res = (n % M * ((n + 1) % M)) % M * 2usize.inverse_mod(M).unwrap() % M;
    let mut now = 1;
    while now < n {
        let div = n / now;
        let up = n / div;
        res += M - ((up + 1 - now) % M * (div % M)) % M;
        res %= M;
        now = up + 1;
    }

    println!("{}", res)
}
