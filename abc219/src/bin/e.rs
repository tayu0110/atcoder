#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}};

const W: usize = 4;
const N: usize = W * W;

fn check(map: u32) -> bool {
    let mut uf = unionfind::UnionFind::new(N+1);
    for i in 0..W {
        for j in 0..W {
            let now = W * i + j;
            let nc = (map >> now) & 1;
            if i > 0 {
                let up = now - W;
                if (map >> up) & 1 == nc {
                    uf.merge(up, now);
                }
            } else if nc == 0 {
                uf.merge(now, N);
            }

            if j > 0 {
                let left = now - 1;
                if (map >> left) & 1 == nc {
                    uf.merge(left, now);
                }
            } else if nc == 0 {
                uf.merge(now, N);
            }
            
            if i+1 < W {
                let down = now + W;
                if (map >> down) & 1 == nc {
                    uf.merge(down, now);
                }
            } else if nc == 0 {
                uf.merge(now, N);
            }
            
            if j+1 < W {
                let right = now + 1;
                if (map >> right) & 1 == nc {
                    uf.merge(right, now);
                }
            } else if nc == 0 {
                uf.merge(now, N);
            }
        }
    }

    let mut root_one = std::usize::MAX;
    let mut root_zero = std::usize::MAX;
    for i in 0..=N {
        if map & (1 << i) != 0 {
            if root_one == std::usize::MAX {
                root_one = uf.root(i);
            } else {
                if root_one != uf.root(i) {
                    return false;
                }
            }
        } else {
            if root_zero == std::usize::MAX {
                root_zero = uf.root(i);
            } else {
                if root_zero != uf.root(i) {
                    return false;
                }
            }
        }
    }

    true
}

fn main() {
    input! {a: [u32; N]}

    let mut map = 0;
    for (i, c) in a.into_iter().enumerate() {
        map |= c << i;
    }

    let mut res = 0;
    for i in 0..(1 << N) {
        if i & map != map {
            continue;
        }

        if check(i) {
            res += 1;
        }
    }

    println!("{}", res);
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