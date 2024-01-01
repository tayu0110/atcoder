use proconio::*;

const MOD: usize = 1000_000_007;

fn solve(now: usize, par: usize, t: &Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
    let mut memo = vec![vec![vec![0; 2]; 2]; 2];
    memo[0][0][0] = 1;
    memo[1][1][1] = 1;
    for &to in &t[now] {
        if to == par {
            continue;
        }

        let res = solve(to, now, t);
        let mut new = vec![vec![vec![0; 2]; 2]; memo.len() + res.len()];
        for x in 0..memo.len() {
            for px in 0..2 {
                for cx in 0..2 {
                    for y in 0..res.len() {
                        for py in 0..2 {
                            for cy in 0..2 {
                                let mut next = x + y;
                                let nc = cx | py;
                                next += (cx == 0 && py == 1) as usize;
                                next += (cy == 0 && px == 1) as usize;
                                if next < new.len() {
                                    let t = memo[x][px][cx] * res[y][py][cy] % MOD;
                                    new[next][px][nc] += t;
                                    new[next][px][nc] %= MOD;
                                }
                            }
                        }
                    }
                }
            }
        }

        new.pop();
        memo = new;
    }

    memo
}

fn main() {
    input! {n: usize, e: [(usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
        t[v - 1].push(u - 1);
    }

    let res = solve(0, 0, &t);
    for i in 0..=n {
        println!("{}", (res[i][0][0] + res[i][0][1] + res[i][1][1]) % MOD)
    }
}
