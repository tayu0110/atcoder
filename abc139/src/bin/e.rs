use std::collections::VecDeque;

use proconio::*;

fn main() {
    input! {n: usize, a: [[usize; n-1]; n]}

    let mut a = a
        .into_iter()
        .map(|v| v.into_iter().map(|v| v - 1).collect::<VecDeque<_>>())
        .collect::<Vec<_>>();
    let mut nt = VecDeque::new();
    for i in 0..n {
        let to = a[i][0];
        if to < i {
            continue;
        }

        if a[to][0] == i {
            nt.push_back((i, to));
        }
    }

    let mut res = 1;
    let mut f = vec![false; n];
    while let Some((s, t)) = nt.pop_front() {
        a[s].pop_front();
        a[t].pop_front();
        if f[s] || f[t] {
            res += 1;
            f.iter_mut().for_each(|f| *f = false);
        }
        f[s] = true;
        f[t] = true;

        if !a[s].is_empty() {
            let to = a[s][0];
            if !a[to].is_empty() && a[to][0] == s {
                nt.push_back((s, to));
            }
        }
        if !a[t].is_empty() {
            let to = a[t][0];
            if to != s && !a[to].is_empty() && a[to][0] == t {
                nt.push_back((t, to));
            }
        }
    }

    if a.iter().all(|v| v.is_empty()) {
        println!("{}", res)
    } else {
        println!("-1")
    }
}
