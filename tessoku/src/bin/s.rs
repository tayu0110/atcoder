use proconio::input;

fn main() {
    input! {n: usize, w: usize, p: [(usize, usize); n]}

    let mut dp = vec![0; w + 1];
    for (nw, nv) in p {
        for i in (0..w).rev() {
            if i + nw <= w {
                dp[i + nw] = dp[i + nw].max(dp[i] + nv);
            }
        }
    }

    println!("{}", dp.iter().max().unwrap())
}
