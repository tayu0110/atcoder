#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m]}

    if m != n - 1 {
        println!("No");
        return;
    }

    let mut uf = unionfind::UnionFind::new(n);
    let mut ins = vec![0; n];

    for (u, v) in p {
        ins[u - 1] += 1;
        ins[v - 1] += 1;
        uf.merge(v - 1, u - 1);
    }

    let mut k = 0;
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        if ins[i] != 1 && ins[i] != 2 {
            println!("No");
            return;
        }
        if ins[i] == 1 {
            k += 1;
        }
        set.insert(uf.root(i));
    }

    if set.len() == 1 && k == 2 {
        println!("Yes")
    } else {
        println!("No")
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
