use proconio::*;

fn main() {
    input! {n: usize, h: usize, mut x: [usize; n], mut e: [(usize, usize); n-1]}
    e.push((0, 0));

    let mut dp = vec![vec![usize::MAX; h + 1]; h + 1];
    dp[h][0] = 0;

    x.insert(0, 0);
    for (i, x) in x.windows(2).enumerate() {
        let (p, f) = e[i];
        let dist = x[1] - x[0];

        if dist > h {
            println!("-1");
            return;
        }

        let mut new = vec![vec![usize::MAX; h + 1]; h + 1];
        for i in 0..=h {
            for j in 0..=h {
                if dp[i][j] == usize::MAX {
                    continue;
                }

                if i < dist || j + dist > h {
                    continue;
                }

                // not use
                new[i - dist][j + dist] = new[i - dist][j + dist].min(dp[i][j]);

                // outward
                {
                    let next = (i - dist + f).min(h);
                    new[next][j + dist] = new[next][j + dist].min(dp[i][j] + p);
                }

                // return
                {
                    let next = (j + dist).saturating_sub(f);
                    new[i - dist][next] = new[i - dist][next].min(dp[i][j] + p);
                }
            }
        }

        dp = new;
    }

    let mut res = usize::MAX;
    for i in 0..=h {
        for j in 0..=h {
            if i >= j {
                res = res.min(dp[i][j]);
            }
        }
    }
    println!("{}", res as i64);
}
