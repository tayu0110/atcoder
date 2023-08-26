use proconio::*;

fn main() {
    input! {n: usize, m: usize}

    let mut s = vec![];
    let mut g = vec![vec![]; m];
    for i in 0..n {
        input! {a: usize, mut ns: [usize; a]}
        for s in ns.iter_mut() {
            *s -= 1;
            g[*s].push(i);
        }
        s.push(ns);
    }

    let mut nt = vec![0];
    let mut dist = vec![std::usize::MAX; m];
    let mut reached_group = vec![false; n];
    for i in 0.. {
        let mut new = vec![];
        for now in nt {
            for &group in &g[now] {
                if reached_group[group] {
                    continue;
                }

                reached_group[group] = true;

                for &to in &s[group] {
                    if dist[to] == std::usize::MAX {
                        dist[to] = i;
                        new.push(to);
                    }
                }
            }
        }

        nt = new;

        if nt.is_empty() {
            break;
        }
    }

    if dist[m - 1] == std::usize::MAX {
        println!("-1")
    } else {
        println!("{}", dist[m - 1])
    }
}
