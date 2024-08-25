use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(u32, u16, u16); n]}

    let mut t = vec![vec![]; 5001];
    for (a, b, c) in p {
        t[b as usize].push((a, c));
    }

    let mut dp = vec![u32::MAX; m + 1];
    dp[0] = 0;
    for t in t {
        if !t.is_empty() {
            for i in (0..=m).rev() {
                if dp[i] == u32::MAX {
                    continue;
                }

                for &(a, c) in &t {
                    dp[(i + c as usize).min(m)] = dp[(i + c as usize).min(m)].min(dp[i] + a);
                }
            }
        }
    }

    println!("{}", dp[m] as i32)
}
