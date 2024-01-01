use proconio::*;

fn main() {
    input! {n: usize, a: [u32; n]}

    let mut dp = vec![u32::MAX; n];
    for a in a {
        let (mut l, mut r) = (-1, n as i32);
        while r - l > 1 {
            let m = (r + l) / 2;
            if dp[m as usize] < a {
                l = m;
            } else {
                r = m;
            }
        }

        dp[r as usize] = a;
    }

    println!("{}", dp.iter().take_while(|&&i| i < u32::MAX).count())
}
