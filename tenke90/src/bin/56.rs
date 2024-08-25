use proconio::input;

fn main() {
    input! {n: usize, s: usize, p: [[usize; 2]; n]};

    const INF: usize = 111222333444;

    let mut dp = vec![vec![INF; s+1]; n+1];
    dp[0][0] = 0;

    for (i, v) in p.iter().enumerate() {
        for j in 0..s+1 {
            if dp[i][j] != INF {
                for (k, w) in v.iter().enumerate() {
                    if j + *w <= s {
                        dp[i+1][j + *w] = k;
                    }
                }
            }
        }
    }

    let mut n = n;
    let mut now = s;
    let mut res = vec![];
    while now > 0 {
        if dp[n][now] == INF {
            println!("Impossible");
            std::process::exit(0);
        }
        
        res.push(dp[n][now]);
        now -= p[n-1][dp[n][now]];
        n -= 1;
    }

    for v in res.iter().rev() {
        if *v == 0 {
            print!("A");
        } else {
            print!("B");
        }
    }
    println!();
}