use proconio::*;

fn main() {
    input! {t: usize}

    for _ in 0..t {
        input! {mut n: usize, m: usize, mut a: [usize; n]}
        if n % 2 == 1 {
            a.remove(n / 2);
            n -= 1;
        }

        if n == 0 {
            println!("0");
            continue;
        }

        let mut t = vec![];
        for i in 0..n / 2 {
            t.push((a[n - 1 - i] + m - a[i]) % m);
        }

        // 0: addition, 1: subtraction
        let mut dp = vec![[usize::MAX; 2]; n / 2 + 1];
        dp[0][0] = m - t[0];
        dp[0][1] = t[0];

        for i in 1..n / 2 {
            dp[i][0] = dp[i - 1][1] + m - t[i];
            dp[i][1] = dp[i - 1][0] + t[i];
            if t[i - 1] <= t[i] {
                dp[i][0] = dp[i][0].min(dp[i - 1][0]);
                dp[i][1] = dp[i][1].min(dp[i - 1][1] + t[i] - t[i - 1]);
            } else {
                dp[i][0] = dp[i][0].min(dp[i - 1][0] + t[i - 1] - t[i]);
                dp[i][1] = dp[i][1].min(dp[i - 1][1]);
            }
        }

        println!("{}", dp[n / 2 - 1].iter().min().unwrap());
    }
}
