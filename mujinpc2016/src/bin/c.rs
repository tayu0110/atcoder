use std::collections::HashSet;

use proconio::*;

fn set(l: &mut u128, r: &mut u128, i: usize) {
    if i < 128 {
        *l |= 1 << i;
    } else {
        *r |= 1 << (i - 128);
    }
}

fn solve(n: usize, b: usize, t: &[Vec<(usize, usize)>]) -> Option<(u128, u128)> {
    let mut uf = unionfind::UnionFind::new(n);
    let mut res = (0, 0);
    for i in 0..n {
        let pb = (b >> i) & 1;
        for &(to, index) in &t[i] {
            let nb = (b >> to) & 1;
            if pb != nb {
                uf.merge(i, to);
                set(&mut res.0, &mut res.1, index);
            }
        }
    }

    for i in 0..n {
        let pb = (b >> i) & 1;
        for &(to, _) in &t[i] {
            let nb = (b >> to) & 1;
            if pb == nb && !uf.is_same(i, to) {
                return None;
            }
        }
    }

    Some(res)
}

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    for (i, (x, y)) in p.into_iter().enumerate() {
        t[x - 1].push((y - 1, i));
        t[y - 1].push((x - 1, i));
    }

    let mut res = HashSet::new();
    for i in 0..1 << n {
        if let Some(r) = solve(n, i, &t) {
            res.insert(r);
        }
    }

    println!("{}", res.len());
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
