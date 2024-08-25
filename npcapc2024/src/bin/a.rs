use math::MathInt;
use proconio::*;

fn main() {
    input! {t: usize, query: [usize; t]}

    const M: usize = 998244353;
    const LEN: usize = "NPCAPC".len();

    let mut inv = 1;
    for i in 1..=LEN {
        inv *= i.inverse_mod(M).unwrap().pow_mod(2, M);
        inv %= M;
    }

    for n in query {
        if n < LEN * 2 {
            println!("0");
            continue;
        }

        let mut res = inv;
        for i in (1..=n).rev().take(LEN * 2) {
            res *= i % M;
            res %= M;
        }

        res *= 51usize.pow_mod((n - LEN * 2) as u64, M);
        res %= M;

        println!("{res}")
    }
}
