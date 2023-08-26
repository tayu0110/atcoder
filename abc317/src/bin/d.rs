use proconio::*;

fn main() {
    input! {n: usize, p: [(i32, i32, usize); n]}

    let np = p
        .into_iter()
        .map(|(a, b, c)| {
            let half = (a + b + 1) / 2;
            ((half - a).max(0) as usize, c)
        })
        .collect::<Vec<_>>();
    let sum = np.iter().map(|(_, c)| c).sum::<usize>();

    let mut dp = vec![usize::MAX; sum + 1];
    dp[0] = 0;
    for (d, c) in np {
        for now in (0..sum + 1).rev() {
            if now + c > sum {
                continue;
            }
            if dp[now] == usize::MAX {
                continue;
            }

            dp[now + c] = dp[now + c].min(dp[now] + d);
        }
    }

    println!("{}", dp[(sum + 1) / 2..].iter().min().unwrap())
}
