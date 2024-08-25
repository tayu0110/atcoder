use proconio::*;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {n: usize}

    let mut res = 1;
    for i in 1..=n {
        res *= i;
        res %= MOD;
    }

    println!("{}", res)
}
