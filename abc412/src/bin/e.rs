use math::MathInt;
use proconio::*;

fn main() {
    input! {l: usize, r: usize}

    let mut f = l.is_prime();
    let mut res = (l..=r).filter(|&l| l.is_prime()).count();
    for i in (2..).take_while(|&i| i * i <= r) {
        if i.is_prime() {
            let mut now = i * i;
            while now <= r {
                if (l..=r).contains(&now) {
                    res += 1;
                    f |= now == l;
                }
                now = now.saturating_mul(i);
            }
        }
    }

    if !f {
        res += 1;
    }
    println!("{res}")
}
