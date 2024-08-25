use proconio::*;

const MAX: usize = 130;

fn main() {
    input! {n: usize}

    let n = n.to_string().chars().collect::<Vec<_>>();

    let mut dp = vec![vec![vec![vec![0usize; 2]; MAX]; MAX]; MAX];
    for i in 1..MAX {
        dp[i][0][0][1] = 1;
    }

    for &c in &n {
        let mut new = vec![vec![vec![vec![0; 2]; MAX]; MAX]; MAX];
        let c = c as usize - b'0' as usize;
        for d in 0..10 {
            for f in 0..2 {
                if f == 1 && d > c {
                    continue;
                }
                for modulo in 1..MAX {
                    for m in 0..modulo {
                        for k in 0..MAX {
                            if dp[modulo][m][k][f] == 0 {
                                continue;
                            }

                            let nm = (m * 10 + d) % modulo;
                            let nk = k + d;
                            if f == 0 {
                                new[modulo][nm][nk][0] += dp[modulo][m][k][f];
                            } else if d == c {
                                new[modulo][nm][nk][1] += dp[modulo][m][k][f];
                            } else {
                                new[modulo][nm][nk][0] += dp[modulo][m][k][f];
                            }
                        }
                    }
                }
            }
        }
        dp = new;
    }

    let mut res = 0;
    for modulo in 1..MAX {
        res += dp[modulo][0][modulo][0] + dp[modulo][0][modulo][1];
    }

    println!("{res}")
}
