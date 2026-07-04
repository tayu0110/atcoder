use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {t: usize}
    for _ in 0..t {
        input! {n: usize, m: usize}

        let mut ret = 0usize;
        let mut ten = 1;
        for _dy in 1..20 {
            if ten > n {
                break;
            }
            let nten = ten * 10;
            let ny = nten.min(n + 1) - ten;
            if (nten - 1) % m == 0 {
                ret += (ny % M) * (n % M) % M;
                ret %= M;
            } else {
                let gcd = (nten - 1).gcd(m);
                ret += (ny % M) * (n / (m / gcd) % M) % M;
                ret %= M;
            }
            // eprintln!("dy: {_dy}, ny: {ny}, nten: {nten}, ret: {ret}");

            ten = nten;
        }
        println!("{ret}");
    }
}
