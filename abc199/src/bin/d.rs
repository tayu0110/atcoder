use proconio::*;
use unionfind::UnionFind;

fn rec(now: usize, state: &mut [u8], t: &Vec<Vec<usize>>) -> usize {
    if now == state.len() {
        return 1;
    }

    if t[now].len() == 0 {
        return 3 * rec(now + 1, state, t);
    }

    let mut res = 0;
    'base: for i in 1..=3 {
        for &to in &t[now] {
            if state[to] == i {
                continue 'base;
            }
        }

        state[now] = i;
        res += rec(now + 1, state, t);
        state[now] = 0;
    }

    res
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for (a, b) in p {
        t[a - 1].push(b - 1);
        t[b - 1].push(a - 1);
        uf.merge(a - 1, b - 1);
    }

    let mut nt = vec![vec![]; n];
    for i in 0..n {
        nt[uf.root(i)].push(i);
    }

    let mut res = 1;
    for i in 0..n {
        if nt[i].len() > 0 {
            let mut state = vec![0; nt[i].len()];
            let mut v = vec![vec![]; nt[i].len()];
            for &now in &nt[i] {
                let pos = nt[i].iter().position(|&v| v == now).unwrap();
                for &to in &t[now] {
                    let np = nt[i].iter().position(|&v| v == to).unwrap();
                    v[pos].push(np);
                }
            }

            res *= rec(0, &mut state, &v);
        }
    }

    println!("{}", res)
}
