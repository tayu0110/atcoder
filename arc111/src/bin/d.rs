use proconio::*;

fn dfs(now: usize, t: &Vec<Vec<(usize, usize)>>, reached: &mut [bool], res: &mut [(usize, usize)]) {
    reached[now] = true;
    for &(to, index) in &t[now] {
        if res[index] != (std::usize::MAX, std::usize::MAX) {
            continue;
        }

        res[index] = (now + 1, to + 1);
        if reached[to] {
            continue;
        }

        dfs(to, t, reached, res);
    }
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], c: [usize; n]}

    let mut res = vec![(std::usize::MAX, std::usize::MAX); m];
    let mut t = vec![vec![]; n];
    for (i, &(a, b)) in p.iter().enumerate() {
        if c[a - 1] != c[b - 1] {
            if c[a - 1] > c[b - 1] {
                res[i] = (a, b);
            } else {
                res[i] = (b, a);
            }
        } else {
            t[a - 1].push((b - 1, i));
            t[b - 1].push((a - 1, i));
        }
    }

    let mut reached = vec![false; n];
    for i in 0..n {
        dfs(i, &t, &mut reached, &mut res);
    }

    for (i, r) in res.into_iter().enumerate() {
        if p[i] == r {
            println!("->")
        } else {
            println!("<-")
        }
    }
}
