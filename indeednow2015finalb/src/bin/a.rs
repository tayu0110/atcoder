use proconio::*;

const MOD: usize = 1000_000_007;

fn main() {
    input! {a: usize, b: usize}

    let mut res = 0;
    for i in a..=b {
        res += i * i * (i + 1) / 2;
        res %= MOD;
    }

    println!("{}", res)
}
