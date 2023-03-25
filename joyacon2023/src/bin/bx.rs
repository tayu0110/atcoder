use proconio::input;

fn main() {
    const MOD: u128 = 998244353;
    input! {n: u128}

    let mut res = 0;
    let mut ten = 1u128;
    while ten <= n {
        let next_ten = ten * 10;
        let diff = if next_ten <= n {
            (next_ten - ten) % MOD
        } else {
            (n - ten + 1) % MOD
        };
        res += diff * (diff + 1) / 2;
        res %= MOD;
        ten = next_ten;
    }

    println!("{}", res);
}
