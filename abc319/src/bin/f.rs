use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::*;

fn main() {
    input! {n: usize, mut e: [(usize, usize, i64, i64); n - 1]}
    e.iter_mut().for_each(|e| e.0 -= 1);
    e.insert(0, (0, 1, 0, 0));

    let mut t = vec![vec![]; n];
    let mut drug = vec![];
    let mut map = vec![usize::MAX; n];
    for (i, &(p, ty, s, g)) in e.iter().enumerate().skip(1) {
        t[p].push(i);
        if ty == 2 {
            map[i] = drug.len();
            drug.push((i, s, g));
        }
    }
    let m = drug.len();

    let mut dp = vec![-1; 1 << m];
    let mut nt = BinaryHeap::new();
    nt.push((1i64, 0));
    while let Some((nd, now)) = nt.pop() {
        if nd <= dp[now] {
            continue;
        }
        dp[now] = nd;

        let mut e = e.clone();
        if now > 0 {
            let mut nt = vec![0];
            while let Some(node) = nt.pop() {
                let (_, ty, s, _) = e[node];
                if nd < s {
                    continue;
                }
                if ty == 2 && now & (1 << map[node]) == 0 {
                    continue;
                }

                for &to in &t[node] {
                    nt.push(to);
                }

                e[node].1 = 1;
                e[node].2 = 0;
                e[node].3 = 0;
            }
        }

        let mut buf = vec![];
        let mut reached = vec![false; n];
        let mut strength = nd;
        {
            let mut nt = BinaryHeap::new();
            nt.push(Reverse((0, 0)));
            while let Some(Reverse((nd, now))) = nt.pop() {
                if reached[now] {
                    continue;
                }
                reached[now] = true;
                let (_, ty, _, g) = e[now];
                if ty == 1 {
                    if nd <= strength {
                        strength = strength.saturating_add(g);
                    } else {
                        reached[now] = false;
                        continue;
                    }
                } else {
                    buf.push(map[now]);
                    reached[now] = false;
                    continue;
                }

                for &to in &t[now] {
                    if !reached[to] {
                        let (_, _, s, _) = e[to];
                        nt.push(Reverse((s, to)));
                    }
                }
            }
        }

        if now == (1 << m) - 1 && reached.iter().all(|&f| f) {
            println!("Yes");
            return;
        }

        for b in buf {
            let mut strength = strength.saturating_mul(drug[b].2);
            let mut reached = reached.clone();
            reached[drug[b].0] = true;
            {
                let mut nt = BinaryHeap::new();
                for i in 0..n {
                    if reached[i] {
                        for &to in &t[i] {
                            if !reached[to] {
                                let (_, ty, s, _) = e[to];
                                if ty == 1 {
                                    nt.push(Reverse((s, to)));
                                }
                            }
                        }
                    }
                }
                while let Some(Reverse((nd, now))) = nt.pop() {
                    if reached[now] {
                        continue;
                    }
                    reached[now] = true;
                    let (_, _, _, g) = e[now];
                    if strength < nd {
                        continue;
                    }
                    strength = strength.saturating_add(g);
                    for &to in &t[now] {
                        let (_, ty, s, _) = e[to];
                        if !reached[to] && ty == 1 {
                            nt.push(Reverse((s, to)));
                        }
                    }
                }
            }
            nt.push((strength, now | (1 << b)));
        }
    }

    println!("No");
}
