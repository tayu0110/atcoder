use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {mut k: usize, n: usize, a: [(usize, usize); n]}

    if n == 1 {
        let (a, d) = a[0];
        println!("{}", a * k + k * k.saturating_sub(1) / 2 * d);
        return;
    }

    let (mut l, mut r) = (0, std::usize::MAX >> 10);
    while r - l > 1 {
        let m = (r + l) / 2;

        let mut cnt = 0;
        for &(a, d) in &a {
            if a < m {
                cnt += (m - a + d - 1) / d;
            }
        }

        if cnt > k {
            r = m;
        } else {
            l = m;
        }
    }

    let mut res = 0;
    let mut nt = BinaryHeap::new();
    for &(mut a, d) in &a {
        if a < l {
            let cnt = (l - 1 - a) / d;
            res += cnt * a + cnt.saturating_sub(1) * cnt / 2 * d;
            k = k.checked_sub(cnt).unwrap();
            a += cnt * d;
        }
        nt.push(Reverse((a, d)));
    }

    while k > 0 {
        let Reverse(f) = nt.pop().unwrap();
        res += f.0;
        nt.push(Reverse((f.0 + f.1, f.1)));
        k -= 1;
    }

    println!("{}", res)
}
