use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut dp = vec![std::usize::MAX; 1 << n];
    dp[0] = 0;
    for _ in 0..m {
        input! {a: usize, b: usize, c: [usize; b]}

        for i in (0..1 << n).rev() {
            if dp[i] < std::usize::MAX {
                let mut next = i;
                for &c in &c {
                    next |= 1 << (c - 1);
                }

                dp[next] = dp[next].min(dp[i] + a);
            }
        }
    }

    let res = dp[(1 << n) - 1];
    if res == std::usize::MAX {
        println!("-1")
    } else {
        println!("{}", dp[(1 << n) - 1])
    }
}
