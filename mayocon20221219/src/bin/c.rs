#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    const MOD: u128 = 998244353;
    input! {n: u128}

    let mut now = 1;
    let mut res = 0;

    while now <= n {
        let ten = now * 10;
        let k = if ten <= n { ten - now } else { n - now + 1 };

        res += k * (k + 1) / 2;
        res %= MOD;

        now *= 10;
    }

    println!("{}", res);
}
