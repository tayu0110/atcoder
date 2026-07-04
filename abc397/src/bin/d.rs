use math::MathInt;
use proconio::*;

fn main() {
    input! {n: u64}

    let divisors = n.divisors();
    for s in divisors {
        let t = n / s;

        let (mut l, mut r) = (0, 1000_000_001);
        while r - l > 1 {
            let x = (r + l) / 2;
            if x < s {
                l = x;
                continue;
            }

            let y = x - s;
            let nt = x
                .saturating_mul(x)
                .saturating_add(x.saturating_mul(y))
                .saturating_add(y.saturating_mul(y));
            if nt > t {
                r = x;
            } else {
                l = x;
            }
        }

        if l <= s {
            continue;
        }

        let x = l;
        let y = l - s;
        let nt = x
            .saturating_mul(x)
            .saturating_add(x.saturating_mul(y))
            .saturating_add(y.saturating_mul(y));
        if nt == t {
            println!("{x} {y}");
            return;
        }
    }

    println!("-1")
}
