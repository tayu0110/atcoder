use proconio::*;

fn main() {
    input! {n: usize, x: usize, p: [(i32, usize, i32); n]}

    let mut dp = vec![(i32::MIN, i32::MIN); x + 1];
    dp[x] = (0, 10i32.pow(9));
    let mut res = (0, 0, 0);
    for (a, b, c) in p {
        for i in b..=x {
            if dp[i] == (i32::MIN, i32::MIN) {
                continue;
            }

            let (g, s) = dp[i];
            dp[i - b] = dp[i - b].max((g + c, s - a));
        }
        let (g, s) = dp.iter().cloned().max().unwrap();
        let b = dp.iter().rposition(|d| d == &(g, s)).unwrap();
        res = res.max((g, s, b));
    }

    println!("{} {} {}", res.0, res.1, res.2);
}
