use std::collections::HashSet;
use proconio::input;

fn main() {
    input! {n: usize, m: usize, q: usize, p: [(usize, usize); m], query: [(usize, usize); q]};

    let mut t = vec![vec![]; n];
    let mut ck = HashSet::new();
    for (x, y) in p {
        if ck.contains(&(x, y)) {
            continue;
        }
        if x == y {
            continue;
        }
        ck.insert((x, y));
        t[y-1].push(x-1);
    }
    
    for i in (0..q).step_by(128) {
        let mut dp = vec![0u128; n];
        let mx = std::cmp::min(q, i+128);
        for j in i..mx {
            let (a, _) = query[j];
            dp[a-1] |= 1u128 << (j % 128);
        }

        for j in 0..n {
            for to in &t[j] {
                let tmp = dp[*to];
                dp[j] |= tmp;
            }
        }

        for j in i..mx {
            let (_, b) = query[j];
            if (dp[b-1] & (1u128 << (j % 128))) != 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}