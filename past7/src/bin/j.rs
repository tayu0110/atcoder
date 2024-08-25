use proconio::*;

fn dfs(now: usize, state: &mut [u8], t: &[Vec<usize>]) -> bool {
    state[now] = 1;
    for &to in &t[now] {
        if state[to] == 1 {
            return true;
        }

        if state[to] == 0 {
            let r = dfs(to, state, t);
            if r {
                return true;
            }
        }
    }

    state[now] = 2;
    false
}

fn main() {
    input! {n: usize, m: usize, e: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (u, v) in e {
        t[u - 1].push(v - 1);
    }

    // 0: undecided, 1: check, 2: finish
    let mut state = vec![0; n];
    for i in 0..n {
        if state[i] == 2 {
            continue;
        }

        let res = dfs(i, &mut state, &t);
        if res {
            println!("Yes");
            return;
        }
    }

    println!("No")
}
