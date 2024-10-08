use math::{divisors_enumeration, factorize};
use proconio::*;
use static_modint::{combination, Mod1000000007, StaticModint};

type M = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, m: usize}

    if m == 1 {
        println!("1");
        return;
    }

    let mut f = factorize(m as u64);
    f.sort();

    let mut d = divisors_enumeration(m as u64);
    d.sort();

    let mut dp = vec![vec![M::zero(); d.len()]; f.len() + 1];
    dp[0][d.len() - 1] = M::one();
    for i in 0..f.len() {
        for j in 1..d.len() {
            if dp[i][j] == M::zero() {
                continue;
            }
            let mut now = j;
            for k in 1..=j {
                if d[j] % d[k] != 0 {
                    continue;
                }

                let next = d[j] / d[k];
                while d[now] != next {
                    now -= 1;
                }
                dp[i + 1][now] = dp[i + 1][now] + dp[i][j];
            }
        }
    }

    let com = combination::<Mod1000000007>(n as u32 + 10);
    let mut res = M::zero();
    for (i, f) in dp.iter().enumerate().skip(1).take(f.len()) {
        if i > n {
            break;
        }

        res += com(n as u32, i as u32) * f[0];
    }

    println!("{}", res);
}
