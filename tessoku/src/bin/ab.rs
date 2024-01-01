use itertools::Itertools;
use proconio::*;

const MOD: u32 = 10000;

fn main() {
    input! {n: usize, p: [(char, u32); n]}

    println!(
        "{}",
        p.into_iter()
            .scan(0, |s, (t, a)| {
                Some({
                    if t == '+' {
                        *s += a;
                    } else if t == '-' {
                        *s += MOD - a
                    } else {
                        *s *= a
                    }
                    *s %= MOD;
                    *s
                })
            })
            .join("\n")
    );
}
