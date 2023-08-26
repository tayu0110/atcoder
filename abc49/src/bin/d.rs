use std::collections::HashMap;

use proconio::*;

fn main() {
    input! {n: usize, k: usize, l: usize, p: [(usize, usize); k], q: [(usize, usize); l]}

    let mut uf_load = unionfind::UnionFind::new(n);
    let mut uf_rail = unionfind::UnionFind::new(n);

    for (p, q) in p {
        uf_load.merge(p - 1, q - 1);
    }

    for (r, s) in q {
        uf_rail.merge(r - 1, s - 1);
    }

    let mut map = HashMap::new();
    for i in 0..n {
        *map.entry((uf_load.root(i), uf_rail.root(i))).or_insert(0) += 1;
    }

    for i in 0..n {
        println!("{}", map.get(&(uf_load.root(i), uf_rail.root(i))).unwrap())
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
