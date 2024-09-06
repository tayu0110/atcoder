use std::cmp::Reverse;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, mut a: [usize; n], mut rooms: [(usize, usize, usize); m]}

    a.sort_unstable_by_key(|&a| Reverse(a));
    rooms.sort_unstable_by_key(|r| Reverse(r.0));

    let mut dp = vec![usize::MAX; n + 1];
    dp[0] = 0;
    for (r, b, c) in rooms {
        for i in (0..n).rev() {
            if dp[i] == usize::MAX {
                continue;
            }

            if a[i] > r {
                continue;
            }

            let next = (i + b).min(n);
            dp[next] = dp[next].min(dp[i] + c);
        }
    }

    println!("{}", dp[n] as i64)
}
