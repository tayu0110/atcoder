use std::{cmp::Reverse, collections::BinaryHeap};

use cpgraph::FixedGraph;
use proconio::*;

fn main() {
    input! {n: usize, m: usize, k: usize, c: [char; n], e: [(usize, usize); m]}

    if m == 0 {
        if k == 1 {
            println!("{}", c.iter().min().unwrap());
        } else {
            println!("-1");
        }
        return;
    }

    let t = FixedGraph::<(), true>::from_edges(e.into_iter().map(|(a, b)| (a - 1, b - 1, ())));
    let mut scc = t.scc();
    let mut buf = vec![];
    let mut group = vec![0; n];
    let mut checked = vec![false; n];
    for i in 0..scc.len() {
        let mut s = vec![];
        for &member in &scc[i] {
            group[member] = i;
            s.push(c[member]);
            checked[member] = true;
        }
        s.sort_unstable();
        buf.push(s.into_iter().collect::<String>());
    }
    for i in 0..n {
        if !checked[i] {
            group[i] = scc.len();
            scc.push(vec![i]);
            buf.push(format!("{}", c[i]));
        }
    }

    let mut new = vec![vec![]; scc.len()];
    let mut ins = vec![0; scc.len()];
    for i in 0..n {
        for e in t.edges(i) {
            if group[i] != group[e.to()] {
                new[group[i]].push(group[e.to()]);
                ins[group[e.to()]] += 1;
            }
        }
    }
    let mut nt = BinaryHeap::new();
    let mut memo = vec![vec!["".to_owned(); k + 1]; n];
    for i in 0..scc.len() {
        new[i].sort_unstable();
        new[i].dedup();

        for j in 0..=buf[i].len().min(k) {
            memo[i][j] = buf[i][..j].to_owned();
            nt.push(Reverse((buf[i][..j].to_owned(), i)));
        }
    }

    while let Some(Reverse((s, now))) = nt.pop() {
        if !memo[now][s.len()].is_empty() && memo[now][s.len()] < s {
            continue;
        }
        for &to in &new[now] {
            for len in 0..=buf[to].len() {
                let next = s.len() + len;
                if next > k {
                    continue;
                }

                if memo[to][next].is_empty() {
                    memo[to][next] = format!("{s}{}", &buf[to][..len]);
                    nt.push(Reverse((memo[to][next].clone(), to)));
                } else if &memo[to][next][..s.len()] > &s
                    || (memo[to][next][..s.len()] == s
                        && memo[to][next][s.len()..] > buf[to][..len])
                {
                    memo[to][next] = format!("{s}{}", &buf[to][..len]);
                    nt.push(Reverse((memo[to][next].clone(), to)));
                }
            }
        }
    }

    if let Some(res) = memo
        .into_iter()
        .filter(|memo| !memo[k].is_empty())
        .map(|mut memo| memo.pop().unwrap())
        .min()
    {
        println!("{res}")
    } else {
        println!("-1")
    }
}
