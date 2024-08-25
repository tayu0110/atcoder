use math::factorize;
use proconio::*;
use static_modint::{Mod998244353, StaticModint};

type Modint = StaticModint<Mod998244353>;

fn main() {
    input! {n: usize, m: u64, a: [u64; n]}

    let factor = factorize(m);
    let mut t = vec![];
    for f in factor {
        match t.last_mut() {
            Some((nc, cnt)) if *nc == f => *cnt += 1,
            _ => t.push((f, 1)),
        }
    }
    let t = t
        .into_iter()
        .map(|(f, cnt)| (f, cnt, f.pow(cnt)))
        .collect::<Vec<_>>();
    let tlen = t.len();

    let mut bits = vec![];
    for &(mut a) in &a {
        let mut b = 0;
        for (j, &(t, cnt, tp)) in t.iter().enumerate() {
            if a % tp == 0 {
                b |= 1 << j;
            }
            for _ in 0..cnt {
                if a % t == 0 {
                    a /= t;
                }
            }
        }

        if a == 1 {
            bits.push(b);
        }
    }

    let mut cnt = vec![0; 1 << tlen];
    for b in bits {
        cnt[b] += 1;
    }

    let mut dp = vec![Modint::zero(); 1 << tlen];
    for i in 0..1 << tlen {
        let mut new = vec![Modint::zero(); 1 << tlen];
        let c = Modint::raw(2).pow(cnt[i]) - Modint::one();
        for j in 0..1 << tlen {
            new[i | j] += dp[j] * c;
            new[j] += dp[j];
        }
        new[i] += c;
        dp = new;
    }

    println!("{}", dp[(1 << tlen) - 1])
}
