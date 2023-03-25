use std::collections::HashSet;

use proconio::*;

fn main() {
    input! {n: usize, m: usize, p: [(usize, char, usize, char); m]}

    let mut uf = unionfind::UnionFind::new(n);
    let mut x = 0;
    for (a, _, c, _) in p {
        if uf.is_same(a - 1, c - 1) {
            x += 1;
        }

        uf.merge(a - 1, c - 1);
    }

    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(uf.root(i));
    }

    println!("{} {}", x, set.len() - x);
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
