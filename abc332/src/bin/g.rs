use proconio::*;

struct FordFulkerson {
    t: Vec<Vec<(usize, usize, i64)>>,
    visited: Vec<bool>,
}

impl FordFulkerson {
    fn new(size: usize) -> Self {
        Self {
            t: vec![vec![]; size],
            visited: vec![false; size],
        }
    }

    fn set_edge(&mut self, from: usize, to: usize, cap: i64) {
        let flen = self.t[from].len();
        let tlen = self.t[to].len();
        self.t[from].push((to, tlen, cap));
        self.t[to].push((from, flen, 0));
    }

    fn dfs(&mut self, now: usize, to: usize, f: i64) -> i64 {
        if now == to {
            return f;
        }
        self.visited[now] = true;

        let len = self.t[now].len();
        for i in 0..len {
            let (to, rev, cap) = self.t[now][i];
            if cap == 0 || self.visited[to] {
                continue;
            }

            let flow = self.dfs(to, to, f.min(cap));

            if flow > 0 {
                self.t[now][i].2 -= flow;
                self.t[to][rev].2 += flow;
                return flow;
            }
        }
        0
    }

    fn flow(&mut self, s: usize, t: usize) -> i64 {
        let mut res = 0;
        loop {
            self.visited.fill(false);
            let f = self.dfs(s, t, i64::MAX >> 10);
            if f == 0 {
                break;
            }
            res += f;
        }
        res
    }
}

fn main() {
    input! {n: usize, m: usize, mut a: [i64; n], mut b: [i64; m]}

    // ..n: a, n..n+m: b, n+m: src, n+m+1: dest
    let mut ff = FordFulkerson::new(n + m + 2);
    let mut res = 0;

    for i in 0..m {
        let mut max = 0;
        for j in 0..n {
            max += (i + i) * (j + 1);
        }

        if max >= b[i] as usize {
            b[i] = 0;
            for j in 0..n {
                let t = ((i + 1) as i64 * (j + 1) as i64).min(a[j]);
                b[i] -= t;
                a[j] -= t;
                res += t;
            }
        }
    }

    for i in 0..n {
        if a[i] > 0 {
            ff.set_edge(n + m, i, a[i]);
        }
        for j in 0..m {
            if b[j] > 0 {
                ff.set_edge(i, n + j, (i + 1) as i64 * (j + 1) as i64);
            }
        }
    }
    for i in 0..m {
        ff.set_edge(n + i, n + m + 1, b[i]);
    }

    println!("{}", res + ff.flow(n + m, n + m + 1));
}
