use proconio::*;

fn main() {
    input! {n: usize, m: usize, a: [i64; n]}

    let mut memo = vec![[-1; 2]; m + 1];
    memo[0][0] = 0;
    for a in a {
        for i in (1..m + 1).rev() {
            let [mut s, mut t] = memo[i];
            if s >= 0 {
                s += a;
            }
            if memo[i][0] >= 0 {
                s = s.max(memo[i][0] + a);
            }
            if memo[i][1] >= 0 {
                s = s.max(memo[i][1] + a);
            }
            t = t.max(memo[i - 1][0]);

            memo[i] = [s, t];
        }
        memo[0][0] += a;
    }

    println!("{}", memo[m][0].max(memo[m][1]))
}
