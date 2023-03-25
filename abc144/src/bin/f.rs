#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]};

    let mut t = vec![vec![]; n];
    for (u, v) in p {
        t[u-1].push(v-1);
    }

    let mut dp = vec![0.0; n];
    for now in (0..n-1).rev() {
        let mut e = 0.0;
        for to in &t[now] {
            e += dp[*to];
        }
        e /= t[now].len() as f64;
        dp[now] = e + 1.0;
    }

    // eprintln!("{:?}", dp);

    let inf: f64 = m as f64;
    let mut res = dp[0];
    for i in 0..n-1 {
        let mut ndp = vec![0.0; n];
        for now in (0..n-1).rev() {
            if i == now && t[now].len() == 1 {
                ndp[now] = inf + 1.0;
                continue;
            }
            let mut e = 0.0;
            let mut max = 0.0;
            let mut unreachable = false;
            for to in &t[now] {
                if ndp[*to] > inf {
                    unreachable = true;
                    continue;
                }
                e += ndp[*to];
                if max < ndp[*to] {
                    max = ndp[*to];
                }
            }

            if unreachable {
                ndp[now] = inf + 1.0;
                continue;
            }

            if now == i {
                e -= max;
                e /= (t[now].len() - 1) as f64;
            } else {
                e /= t[now].len() as f64;
            }

            ndp[now] = e + 1.0;
        }

        if ndp[0] > inf {
            continue;
        }

        // eprintln!("i: {}, ndp: {:?}", i, ndp);

        if res > ndp[0] {
            res = ndp[0];
        }
    }

    println!("{}", res);
}
