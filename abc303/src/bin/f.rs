use std::collections::BinaryHeap;

use proconio::*;

fn main() {
    input! {n: usize, h: usize, e: [(usize, usize); n]}

    let (mut l, mut r) = (0, usize::MAX >> 2);
    while r - l > 1 {
        let m = (r + l) / 2;
        let mut rem = h;
        let mut now = 0;
        let mut nt = BinaryHeap::new();
        for &(t, d) in &e {
            nt.push((t * d, t, d));
        }

        let mut max = 0;
        while now < m {
            while let Some((damage, t, d)) = nt.pop() {
                if m - now < t {
                    max = max.max(d);
                    continue;
                }

                let r = (m - now) + 1 - t;
                if damage.saturating_mul(r)
                    > t.saturating_mul(max)
                        + damage.saturating_mul(r.saturating_mul(r.saturating_sub(1)) / 2)
                {
                    rem = rem.saturating_sub(damage.saturating_mul(r));
                } else {
                    rem = rem.saturating_sub(
                        t.saturating_mul(max)
                            + damage.saturating_mul(r.saturating_mul(r.saturating_sub(1)) / 2),
                    );
                }
                now += r;
                nt.push((damage, t, d));
                break;
            }
        }

        if rem == 0 {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{r}")
}
