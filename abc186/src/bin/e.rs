use math::ext_gcd;
use proconio::*;

fn main() {
    input! {t: usize}

    let mut res = vec![];

    for _ in 0..t {
        input! {n: i64, s: i64, k: i64}

        let (g, p, q) = ext_gcd(n, -k);
        eprintln!("g: {}, p: {}, q: {}", g, p, q);
        if s % g != 0 {
            res.push(-1);
            continue;
        }

        res.push((q * (s / g)).rem_euclid(n));
    }

    for r in res {
        println!("{}", r);
    }
}
