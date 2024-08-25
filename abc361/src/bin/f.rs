use math::MathInt;
use proconio::*;

fn main() {
    input! {n: usize}

    let mut res = 0;
    for i in (2usize..).take_while(|&i| i.saturating_mul(i).saturating_mul(i) <= n) {
        let g = i.factorize().map(|v| v.1).fold(0, MathInt::gcd);

        if g > 1 {
            continue;
        }

        let mut now = i;
        for j in (3..).step_by(2) {
            now = now.saturating_mul(i.saturating_mul(i));
            if now > n {
                res += (j - 3) / 2;
                break;
            }
        }
    }

    println!("{}", res + n.sqrti());
}
