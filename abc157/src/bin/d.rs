#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, k: usize, p: [(usize, usize); m], q: [(usize, usize); k]}

    let mut dir = vec![0; n];
    let mut uf = unionfind::UnionFind::new(n);
    for (a, b) in p {
        uf.merge(a - 1, b - 1);
        dir[a - 1] += 1;
        dir[b - 1] += 1;
    }

    let mut t = vec![vec![]; n];
    for (c, d) in q {
        t[c - 1].push(d - 1);
        t[d - 1].push(c - 1);
    }

    for i in 0..n {
        let mut res = uf.size(i) - dir[i] - 1;
        for &to in &t[i] {
            if uf.is_same(i, to) {
                res -= 1;
            }
        }

        println!("{}", res);
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
