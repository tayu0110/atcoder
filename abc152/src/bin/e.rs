use proconio::*;
use static_modint::{Mod1000000007, StaticModint};

type Modint = StaticModint<Mod1000000007>;

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut t = vec![0u32; 1000001];
    for &a in &a {
        let mut now = a;
        for i in (2..=a).take_while(|&i| i * i <= a) {
            if now % i == 0 {
                let mut cnt = 1;
                while now % i == 0 {
                    now /= i;
                    cnt *= i;
                }

                t[i as usize] = t[i as usize].max(cnt);
            }
        }

        if now > 1 {
            t[now as usize] = t[now as usize].max(now);
        }
    }

    let base = t
        .into_iter()
        .filter(|&v| v != 0)
        .map(Modint::raw)
        .fold(Modint::one(), |s, v| s * v);
    let mut res = Modint::zero();
    for a in a {
        res += base / Modint::raw(a);
    }
    println!("{}", res)
}
