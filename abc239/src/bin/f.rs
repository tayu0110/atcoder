#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, m: usize, mut d: [usize; n], p: [(usize, usize); m]}

    let mut t = vec![vec![]; n];
    let mut uf = unionfind::UnionFind::new(n);
    for (a, b) in p {
        t[a-1].push(b-1);
        t[b-1].push(a-1);
        if d[a-1] == 0 || d[b-1] == 0 {
            println!("-1");
            return;
        }
        if uf.is_same(a-1, b-1) {
            println!("-1");
            return;
        }
        d[a-1] -= 1;
        d[b-1] -= 1;
        uf.merge(a-1, b-1);
    }

    let dsum = d.iter().sum::<usize>();
    if dsum % 2 != 0 || dsum / 2 != n - m - 1 {
        println!("-1");
        return;
    }
    
    let mut g = vec![0; n];
    let mut l = vec![vec![]; n];
    for i in 0..n {
        let root = uf.root(i);
        if d[i] > 0 {
            g[root] += d[i];
            l[root].push((i, d[i]));
        }
    }

    let mut nt = std::collections::BinaryHeap::new();
    for i in 0..n {
        if g[i] > 0 {
            nt.push((g[i], i));
        }
    }

    let mut res = vec![];
    while let Some((mut gv, v)) = nt.pop() {
        if let Some((_, w)) = nt.pop() {
            let mut lv = l[v].pop().unwrap();
            let mut lw = l[w].pop().unwrap();
            res.push((lv.0, lw.0));
            lv.1 -= 1;
            lw.1 -= 1;
            gv -= 1;

            if lv.1 > 0 {
                l[v].push(lv);
            }
            if lw.1 > 0 {
                l[w].push(lw);
            }

            if !l[w].is_empty() {
                while let Some(w) = l[w].pop() {
                    l[v].push(w);
                    gv += w.1;
                }
            }

            uf.merge(lv.1, lw.1);
            if gv != 0 {
                nt.push((gv, v));
            }
        } else {
            println!("-1");
            return;
        }
    }

    for (v, w) in res {
        println!("{} {}", v+1, w+1);
    }
}

#[allow(dead_code)]
mod unionfind {
    pub struct UnionFind {
        tree: Vec<i32>
    }
    impl UnionFind {
        pub fn new(size: usize) -> Self {
            UnionFind { tree: vec![-1; size] }
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
