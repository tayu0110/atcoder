use proconio::*;

fn main() {
    input! {n: usize, d: usize, a: [usize; n]}

    let &max = a.iter().max().unwrap();
    let mut cnt = vec![0; max + 1];
    for &a in &a {
        cnt[a] += 1;
    }

    if d == 0 {
        println!("{}", n - cnt.iter().filter(|&&a| a > 0).count());
        return;
    }

    let mut res = 0usize;
    for i in 0..d {
        let a = cnt.iter().copied().skip(i).step_by(d).collect::<Vec<_>>();
        let n = a.len();
        if n == 0 {
            continue;
        }
        let mut dp = vec![[0; 2]; n];
        dp[0] = [0, a[0]];
        for i in 0..n - 1 {
            dp[i + 1][1] = dp[i][0] + a[i + 1];
            dp[i + 1][0] = dp[i][0].max(dp[i][1]);
        }
        res += dp[n - 1][0].max(dp[n - 1][1]);
    }

    println!("{}", n - res)
}
