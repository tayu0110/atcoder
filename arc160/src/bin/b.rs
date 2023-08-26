use num::integer::Roots;
use proconio::*;

const MOD: usize = 998244353;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {n: usize}

        let m = n.sqrt() + 1;
        let mut res = 0;
        for y in 1..=m {
            if y * y > n {
                break;
            }

            let z = n / y;
            res += (y - 1) * (z - y) * 6;
            res %= MOD;

            res += (z - y) * 3;
            res %= MOD;

            res += (y - 1) * 3;
            res %= MOD;

            res += 1;
            res %= MOD;
        }

        println!("{}", res);
    }
}
