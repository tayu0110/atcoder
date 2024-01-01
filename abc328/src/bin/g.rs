use proconio::*;

fn main() {
    input! {n: usize, c: usize, a:[usize; n], b: [usize; n]}

    let mut dp = vec![usize::MAX; 1 << n];
    dp[0] = 0;
    for i in 0usize..1 << n {
        if dp[i] == usize::MAX {
            continue;
        }

        let start = i.count_ones() as usize;
        for j in 0..n {
            let mut next = i;
            let mut sum = dp[i];
            for k in j..n {
                if i & (1 << k) != 0 {
                    break;
                }

                next |= 1 << k;
                sum += b[k].abs_diff(a[start + k - j]);
                dp[next] = dp[next].min(sum + c);
            }
        }
    }

    println!("{}", dp[(1 << n) - 1] - c)
}
