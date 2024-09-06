use proconio::*;

fn dfs(
    now: usize,
    par: usize,
    res: &mut Vec<(usize, Vec<usize>)>,
    t: &[Vec<(usize, usize)>],
) -> (usize, Vec<usize>) {
    if t[now].len() == 1 && t[now][0].0 == par {
        return (0, vec![now]);
    }

    let (mut w, mut r) = (0, vec![]);
    for &(to, l) in &t[now] {
        if par == to {
            continue;
        }

        let (weight, verts) = dfs(to, now, res, t);
        if w < weight + l {
            if !r.is_empty() {
                res.push((w, r));
            }
            w = weight + l;
            r = verts;
        } else {
            res.push((weight + l, verts));
        }
    }

    r.push(now);
    (w, r)
}

fn main() {
    input! {n: usize, e: [(usize, usize, usize); n - 1]}

    let mut t = vec![vec![]; n];
    for (u, v, l) in e {
        t[u - 1].push((v - 1, l));
        t[v - 1].push((u - 1, l));
    }

    let mut groups = vec![];
    let (w, r) = dfs(0, 0, &mut groups, &t);
    groups.push((w, r));
    groups.sort_unstable();

    let mut res = 0;
    for _ in 0..n {
        if let Some((w, _)) = groups.pop() {
            res += w;
        }
        println!("{}", res * 2)
    }
}
