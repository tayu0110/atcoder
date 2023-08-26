use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], k: usize, cond: [(usize, usize); k], q: usize}

    let mut uf = unionfind::UnionFind::new(n);
    for (u, v) in p {
        uf.merge(u - 1, v - 1);
    }

    let mut group = vec![0; n];
    for i in 0..n {
        group[i] = uf.root(i);
    }

    let mut conds = HashSet::new();
    for (x, y) in cond {
        conds.insert((group[x - 1], group[y - 1]));
        conds.insert((group[y - 1], group[x - 1]));
    }

    for _ in 0..q {
        input! {p: usize, q: usize}

        let (gp, gq) = (group[p - 1], group[q - 1]);

        if conds.contains(&(gp, gq)) {
            println!("No")
        } else {
            println!("Yes")
        }
    }
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
