use proconio::*;

const MOD: u128 = 998244353;

fn main() {
    input! {n: u128}

    let mut res = 0;
    let (mut l, mut u) = (1, 10);
    while l <= n {
        let m = if u <= n { u - l } else { n - l + 1 };
        res += m * (m + 1) / 2 % MOD;
        res %= MOD;
        l *= 10;
        u *= 10;
    }

    println!("{}", res)
}
