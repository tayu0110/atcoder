use proconio::*;
use string::RollingHash;

const M: u32 = 998244353;

fn main() {
    input! {n: usize, t: marker::Bytes}

    let mut dp = vec![0u32; 1 << n];
    dp[(1 << n) - 1] = 1;
    let mut index = Vec::<usize>::with_capacity(n);
    let mut s = String::with_capacity(n);
    let mut buf = vec![];
    for i in (1..1 << n).rev() {
        if dp[i] == 0 {
            continue;
        }

        s.clear();
        index.clear();
        for j in 0..n {
            if i & (1 << j) != 0 {
                s.push(t[j] as char);
                index.push(j);
            }
        }

        let rhash = RollingHash::new(&s);
        for i in 0..s.len() {
            if i == 0 {
                buf.push((rhash.get(1..), index[i]));
            } else if i == s.len() - 1 {
                buf.push((rhash.get(..s.len() - 1), index[i]));
            } else {
                buf.push((rhash.get(..i) + rhash.get(i + 1..), index[i]));
            }
        }

        buf.sort_by_key(|k| (k.0.raw_value(), k.1));
        let mut prev = None;
        for (hash, index) in buf.drain(..) {
            if Some(hash.raw_value()) == prev {
                continue;
            }
            prev = Some(hash.raw_value());

            let next = i ^ (1 << index);
            dp[next] += dp[i];
            if dp[next] >= M {
                dp[next] -= M;
            }
        }
    }

    println!("{}", dp[0]);
}
