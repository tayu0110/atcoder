use proconio::input;

const INF: i64 = 6666555444333222111;

fn dfs(decided: usize, now: usize, n: usize, k: usize, dist: &Vec<i64>, memo: &mut Vec<Vec<i64>>) -> i64 {
    if now == k {
        if decided == ((1 << n) - 1) {
            return 0;
        } else {
            return INF;
        }
    }

    if decided == ((1 << n) - 1) {
        return INF;
    }

    if memo[now][decided] >= 0 {
        return memo[now][decided];
    }

    let rem = ((1 << n) - 1) ^ decided;
    let mut t = rem;
    let mut res = INF;
    while t > 0 {
        let r = std::cmp::max(dist[t], dfs(decided | t, now+1, n, k, dist, memo));
        res = std::cmp::min(r, res);

        t = (t - 1) & rem;
    }

    memo[now][decided] = res;
    res
}

fn main() {
    input! {n: usize, k: usize, p: [(i64, i64); n]};

    let mut dist = vec![0i64; 1 << n];
    for i in 0..(1 << n) {
        let mut buf = vec![];
        for j in 0..n {
            if (i & (1 << j)) != 0 {
                buf.push(j);
            }
        }

        for j in &buf {
            let (x, y) = p[*j];
            for k in &buf {
                if j == k {
                    continue;
                }
                let (nx, ny) = p[*k];
                dist[i] = std::cmp::max(dist[i], (nx - x) * (nx - x) + (ny - y) * (ny - y));
            }
        }
    }

    let mut memo = vec![vec![-1; 1 << n]; k];
    dfs(0, 0, n, k, &dist, &mut memo);

    println!("{}", memo[0][0]);
}