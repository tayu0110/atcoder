use proconio::*;
// use rand::Rng;

fn solve(n: usize, k: usize) -> Vec<(usize, usize)> {
    let mut now = (n - 1) * (n - 2) / 2;
    let mut res = (1..n).map(|v| (0, v)).collect::<Vec<_>>();
    if now == k {
        return res;
    }
    for i in 1..n {
        for j in i + 1..n {
            res.push((i, j));
            now -= 1;
            if now == k {
                return res;
            }
        }
    }
    unreachable!()
}

fn checker(n: usize, e: &[(usize, usize)]) -> usize {
    let mut dp = vec![vec![std::usize::MAX >> 10; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }
    for &(u, v) in e {
        dp[u][v] = 1;
        dp[v][u] = 1;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i][k] + dp[k][j]);
            }
        }
    }

    dp.into_iter().flatten().filter(|&v| v == 2).count() / 2
}

fn main() {
    input! {n: usize, k: usize}
    // use rand::thread_rng;
    // let mut rng = thread_rng();
    // for _ in 0..10000000 {
    //     let n: usize = rng.gen_range(2, 10);
    //     let k: usize = rng.gen_range(0, n * (n - 1) / 2);
    //     eprintln!("n: {n}, k: {k}");

    if (n - 1) * (n - 2) / 2 < k {
        println!("-1");
        return;
        // continue;
    }

    let res = solve(n, k);
    // eprintln!("res: {res:?}");

    assert_eq!(checker(n, &res), k);
    println!("{}", res.len());
    for (a, b) in res {
        println!("{} {}", a + 1, b + 1);
    }
    // }
}
