use proconio::input;
use std::collections::{
    VecDeque, HashSet
};

fn main() {
    input! {n: usize, p: [(usize, usize); n], q: usize, v: [(usize, usize); q]};

    let t = p.into_iter().fold(vec![vec![]; n], |mut t, (u, v)| {
        t[u-1].push(v-1);
        t[v-1].push(u-1);
        t
    });

    let mut out = vec![0; n];
    let mut nt = VecDeque::new();
    let mut cycle = HashSet::new();
    for (i, v) in t.iter().enumerate() {
        out[i] = v.len();
        if out[i] == 1 {
            nt.push_back(i);
        } else {
            cycle.insert(i);
        }
    }

    while let Some(now) = nt.pop_front() {
        out[now] = 0;
        for to in &t[now] {
            if out[*to] > 0 {
                out[*to] -= 1;
                if out[*to] == 1 {
                    nt.push_back(*to);
                    cycle.remove(to);
                }
            }
        }
    }

    const INF: usize = 0x3f3f3f3f3f3f3f3f;
    let mut root = vec![INF; n];
    for v in &cycle {
        let mut nt = VecDeque::new();
        nt.push_back(*v);
        root[*v] = *v;
        while let Some(now) = nt.pop_front() {
            for to in &t[now] {
                if root[*to] == INF && !cycle.contains(to) {
                    nt.push_back(*to);
                    root[*to] = *v;
                }
            }
        }
    }

    for (x, y) in v {
        if root[x-1] == root[y-1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
