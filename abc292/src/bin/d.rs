use std::collections::{HashSet, VecDeque};

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut uf = unionfind::UnionFind::new(n);
    let mut t = vec![vec![]; n];
    for (i, (u, v)) in p.into_iter().enumerate() {
        t[u - 1].push((v - 1, i));
        t[v - 1].push((u - 1, i));
        uf.merge(u - 1, v - 1);
    }

    let mut checked = vec![false; n];
    for i in 0..n {
        if checked[i] {
            continue;
        }

        let size = uf.size(i);

        let mut nt = VecDeque::new();
        let mut edges = HashSet::new();
        nt.push_back(i);
        while let Some(now) = nt.pop_front() {
            if checked[now] {
                continue;
            }
            checked[now] = true;

            for &(to, index) in &t[now] {
                edges.insert(index);

                if !checked[to] {
                    nt.push_back(to);
                }
            }
        }

        if size != edges.len() {
            println!("No");
            return;
        }
    }

    println!("Yes")
}

#[allow(dead_code)]
mod unionfind {
    pub struct UnionFind {
        tree: Vec<i32>,
    }
    impl UnionFind {
        pub fn new(size: usize) -> Self {
            UnionFind {
                tree: vec![-1; size],
            }
        }
        pub fn root(&mut self, index: usize) -> usize {
            if self.tree[index] < 0 {
                index
            } else {
                self.tree[index] = self.root(self.tree[index] as usize) as i32;
                self.tree[index] as usize
            }
        }
        pub fn size(&mut self, index: usize) -> usize {
            let root = self.root(index);
            -self.tree[root] as usize
        }
        pub fn is_same(&mut self, left: usize, right: usize) -> bool {
            self.root(left) == self.root(right)
        }
        pub fn merge(&mut self, left: usize, right: usize) -> bool {
            let (mut rl, mut rr) = (self.root(left), self.root(right));
            if rl == rr {
                return false;
            }
            if self.tree[rl] > self.tree[rr] {
                std::mem::swap(&mut rl, &mut rr);
            }
            self.tree[rl] += self.tree[rr];
            self.tree[rr] = rl as i32;
            true
        }
    }
}
