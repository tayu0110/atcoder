use proconio::*;

const MOD: usize = 1000_000_007;

fn main() {
    input! {n: usize, k: usize}

    let mut res = 1;
    for k in k..=n {
        let a = n - k;
        let max = n * (n + 1) / 2 - a * (a + 1) / 2;
        let min = k * (k - 1) / 2;
        res += max - min + 1;
        res %= MOD;
    }

    println!("{}", res)
}
