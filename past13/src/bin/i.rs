use proconio::*;

fn dfs(now: usize, reached: &mut [bool], t: &Vec<Vec<usize>>) -> bool {
    reached[now] = true;

    let mut f = true;
    for &to in &t[now] {
        if reached[to] {
            return false;
        }

        f &= dfs(to, reached, t);
    }

    reached[now] = false;
    f
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (a, b) in p {
        t[a - 1].push(b - 1);
    }

    for i in 0..n {
        let mut reached = vec![false; n];
        if !dfs(i, &mut reached, &t) {
            println!("No");
            return;
        }
    }

    println!("Yes")
}
