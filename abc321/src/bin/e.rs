use itertools::Itertools;
use proconio::*;

fn solve(n: usize, now: usize, k: usize) -> usize {
    if k > 60 {
        return 0;
    }
    let l = now.saturating_mul(1 << k);
    if l > n {
        return 0;
    }
    let mut r = now;
    for _ in 0..k {
        r = r.saturating_mul(2).saturating_add(1);
    }

    r.min(n) + 1 - l
}

fn main() {
    input! {t: usize}

    let mut r = vec![];
    for _ in 0..t {
        input! {n: usize, x: usize, mut k: usize}

        if k > 120 {
            r.push(0);
            continue;
        }

        let mut res = solve(n, x, k);
        let mut now = x;
        while k > 0 && now > 1 {
            if k == 1 {
                res += 1;
                break;
            }

            res += solve(n, now ^ 1, k - 2);
            k -= 1;
            now >>= 1;
        }

        r.push(res);
    }

    println!("{}", r.iter().join("\n"))
}
