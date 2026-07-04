use math::MathInt;
use proconio::*;

const M: usize = 998244353;

fn main() {
    input! {n: usize, k: usize, a: [usize; n]}

    let mut res = 0;
    let mut sum = 0;
    for (i, &a) in a.iter().enumerate() {
        sum += a;
        let sp = sum.pow_mod(k as u64, M);
        res += sp * (i + 1);
        if k % 2 == 0 {
            res += sp * (n - 1 - i);
        } else {
            res += M - sp * (n - 1 - i) % M;
        }
        res %= M;
    }
    let mut com = 1;
    for l in 1..k {
        com *= k + 1 - l;
        com %= M;
        com *= l.inverse_mod(M).unwrap();
        com %= M;

        let mut b = vec![0; n + 1];
        let mut c = vec![0; n + 1];
        for i in 0..n {
            b[i + 1] = b[i] + a[i];
            b[i + 1] %= M;

            c[i + 1] = c[i] + b[i + 1].pow_mod(l as u64, M);
            c[i + 1] %= M;

            let t = com * b[i + 1].pow_mod((k - l) as u64, M) % M * c[i] % M;
            if l % 2 == 0 {
                res += t;
            } else {
                res += M - t;
            }
            res %= M;
        }
    }

    println!("{res}")
}
