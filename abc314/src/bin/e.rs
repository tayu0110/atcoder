use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut t = vec![];
    for _ in 0..n {
        input! {c: usize, p: usize, s: [usize; p]}
        t.push((c, p, s));
    }

    let mut dp = vec![f64::MAX; m + 200];
    dp[m..m + 200].fill(0.0);
    for i in (0..m).rev() {
        for (c, p, s) in &t {
            let (&c, &p) = (c, p);
            let mut z = 0;
            let mut sum = 0.0;
            for &s in s {
                if s == 0 {
                    z += 1;
                    continue;
                }

                sum += dp[i + s];
            }

            let res = (sum / p as f64 + c as f64) * p as f64 / (p - z) as f64;
            if dp[i] > res {
                dp[i] = res;
            }
        }
    }

    println!("{}", dp[0])
}
