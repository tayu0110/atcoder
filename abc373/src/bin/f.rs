use proconio::*;

fn main() {
    input! {n: usize, w: usize, e: [(usize, i64); n]}

    let mut dp = vec![i64::MIN; w + 1];
    dp[0] = 0;
    for (nw, nv) in e {
        let mut new = vec![i64::MIN; w + 1];
        for i in (0..=w).rev() {
            if dp[i] != i64::MIN {
                for j in 0..=w {
                    let val = j as i64 * nv - j as i64 * j as i64;
                    if i + nw * j > w || val < 0 || dp[i] + val < new[i + nw * j] {
                        break;
                    }
                    new[i + nw * j] = dp[i] + val;
                }
            }
        }
        dp = new;
    }

    println!("{}", dp.into_iter().max().unwrap());
}
