use proconio::*;

fn main() {
    input! {n: usize, a: [usize; n]}

    if n == 2 {
        println!("1");
        return;
    }

    let sum = a.iter().sum::<usize>();
    let mut dp = vec![vec![std::f64::MAX; sum + 1]; sum + 1];
    dp[0][0] = 0.0;
    for _ in 0..n - 2 {
        let mut new = vec![vec![std::f64::MAX; sum + 1]; sum + 1];
        for i in 0..=sum {
            for j in 0..=sum {
                if dp[i][j] > std::f64::MAX - 1000.0 {
                    continue;
                }

                for k in 0..=sum {
                    if j + k > sum {
                        break;
                    }

                    let next = dp[i][j] + 1.0f64.hypot(i.max(k) as f64 - i.min(k) as f64);
                    if next < new[k][j + k] {
                        new[k][j + k] = next;
                    }
                }
            }
        }

        dp = new;
    }

    let mut res = std::f64::MAX;
    for i in 0..=sum {
        let next = dp[i][sum] + 1.0f64.hypot(i as f64);
        if res > next {
            res = next;
        }
    }

    println!("{}", res)
}
