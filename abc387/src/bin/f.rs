use cpgraph::FixedGraph;
use proconio::*;

const M: usize = 998244353;

fn solve(now: usize, m: usize, t: &[Vec<usize>], memo: &mut [Vec<usize>]) {
    if !memo[now].is_empty() {
        return;
    }

    memo[now].resize(m + 1, 1);
    for &to in &t[now] {
        solve(to, m, t, memo);
        for i in 1..m + 1 {
            memo[now][i] *= memo[to][i];
            memo[now][i] %= M;
        }
    }

    for i in (0..m).rev() {
        memo[now][i] += memo[now][i + 1];
        memo[now][i] %= M;
    }
}

fn main() {
    input! {n: usize, m: usize, a: [usize; n]}

    let mut t = vec![vec![]; n];
    for i in 0..n {
        t[i].push(a[i] - 1);
    }

    let graph =
        FixedGraph::<(), true>::from_edges(a.iter().enumerate().map(|(i, a)| (i, a - 1, ())));
    let scc = graph.scc();

    let mut group = vec![0; n];
    for (i, g) in scc.iter().enumerate() {
        for &g in g {
            group[g] = i;
        }
    }

    let mut ins = vec![0; scc.len()];
    let mut t = vec![vec![]; scc.len()];
    for (i, g) in scc.iter().enumerate() {
        for &g in g {
            if group[a[g] - 1] != i {
                t[group[a[g] - 1]].push(i);
                ins[i] += 1;
            }
        }
    }

    let mut memo = vec![vec![]; scc.len()];
    let mut res = 1;
    for i in 0..scc.len() {
        if ins[i] == 0 {
            solve(i, m, &t, &mut memo);
            res *= memo[i][1];
            res %= M;
        }
    }

    println!("{res}")
}
