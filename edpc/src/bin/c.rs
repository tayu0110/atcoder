use proconio::input;

fn main() {
    input! {n: usize, p: [[i32; 3]; n]}

    let mut dp = vec![0; 3];

    for i in 0..n {
        let mut new = vec![std::i32::MIN; 3];
        for j in 0..3 {
            for k in 0..3 {
                if j == k {
                    continue;
                }

                new[k] = std::cmp::max(new[k], dp[j] + p[i][k]);
            }
        }

        dp = new;
    }

    println!("{}", dp.iter().max().unwrap());
}
