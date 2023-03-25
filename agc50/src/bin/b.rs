use proconio::*;

const MIN: i32 = std::i32::MIN >> 5;

fn solve(a: &[i32]) -> Vec<i32> {
    let n = a.len();
    let a = a
        .iter()
        .chain(vec![MIN; 6].iter())
        .cloned()
        .collect::<Vec<_>>();

    let mut dp = vec![vec![MIN; 1 << 6]; n];
    dp[0][0] = 0;
    dp[0][7] = a[0];

    for i in 1..n {
        for j in 0..1 << 6 {
            let mut t = vec![j];
            if j & (0b111 << 3) == 0b111 || j & (0b111 << 3) == 0 {
                t.push(j ^ (0b111 << 3));
            }
            for k in t {
                let next = k >> 1;
                dp[i][next] = dp[i][next].max(dp[i - 1][j] + if next & 1 != 0 { a[i] } else { 0 });
                if next & 0b111 == 0 || next & 0b111 == 0b111 {
                    let next = next ^ 0b111;
                    dp[i][next] =
                        dp[i][next].max(dp[i - 1][j] + if next & 1 != 0 { a[i] } else { 0 });
                }
            }
        }
    }

    let mut res = vec![MIN; n + 6];
    for i in 0..n {
        let mut r = MIN;
        for j in 0..1 << 6 {
            let mut sum = dp[i][j];
            for k in 1..6 {
                if j & (1 << k) != 0 {
                    sum += a[i + k];
                }
            }

            r = r.max(sum);
        }

        res[i + 3] = r;
    }

    res
}

fn main() {
    input! {n: usize, a: [i32; n]}

    let mut dp = vec![vec![0; n + 6]; n];
    for i in 0..n {
        let r = solve(&a[i..]);

        for j in 0..r.len() {
            dp[i][j] = r[j];
        }
    }

    for len in 1..=n {
        for l in 0..n {
            if l + len >= n {
                break;
            }

            for mid in 1..len {
                dp[l][len] = dp[l][len].max(dp[l][mid] + dp[l + mid][len - mid]);
            }
        }
    }

    println!("{}", dp[0][n]);
}
