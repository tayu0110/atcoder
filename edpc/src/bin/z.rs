use proconio::*;
use std::collections::VecDeque;

fn main() {
    input! {n: usize, c: i64, h: [i64; n]}

    let mut nt = VecDeque::new();
    nt.push_back((h[0], h[0] * h[0]));
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for (i, h) in h.into_iter().enumerate().skip(1) {
        let mut prev = (0, std::i64::MAX >> 10);
        while let Some((ph, d)) = nt.pop_front() {
            let new = h * h + c + d - 2 * ph * h;
            if dp[i] >= new {
                dp[i] = new;
                prev = (ph, d);
            } else {
                nt.push_front((ph, d));
                if prev.0 > 0 {
                    nt.push_front(prev);
                }
                break;
            }
        }

        if nt.is_empty() {
            nt.push_back(prev);
        }

        let (k, l) = (h, h * h + dp[i]);

        while nt.len() >= 2 {
            let (s, t) = nt.pop_back().unwrap();
            let (a, b) = nt.pop_back().unwrap();

            nt.push_back((a, b));
            if (t - l) * (a - s) >= (t - b) * (k - s) {
                nt.push_back((s, t));
                break;
            }
        }

        nt.push_back((k, l));
    }

    println!("{}", dp[n - 1]);
}
