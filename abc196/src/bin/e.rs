#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

#[fastout]
fn main() {
    input! {n: usize, p: [(i64, usize); n], q: usize, x: [i64; q]}

    let mut z = x.iter().enumerate().map(|(i, c)| (*c, i)).collect::<Vec<_>>();
    z.sort();

    let mut nt = z.into_iter().collect::<std::collections::VecDeque<_>>();

    let mut uf = unionfind::UnionFind::new(q);

    let mut g = 0;
    for (a, t) in p {
        if t == 1 {
            g += a;
        } else if t == 2 {
            if let Some((c, i)) = nt.pop_front() {
                if c + g > a {
                    nt.push_front((c, i));
                    continue;
                }

                while let Some((c, j)) = nt.pop_front() {
                    if c + g > a {
                        nt.push_front((c, j));
                        break;
                    }

                    uf.merge(i, j);
                }

                nt.push_front((a - g, uf.root(i)));
            }
        } else {
            if let Some((c, i)) = nt.pop_back() {
                if c + g < a {
                    nt.push_back((c, i));
                    continue;
                }

                while let Some((c, j)) = nt.pop_back() {
                    if c + g < a {
                        nt.push_back((c, j));
                        break;
                    }

                    uf.merge(i, j);
                }

                nt.push_back((a - g, uf.root(i)));
            }
        }
    }

    let mut res = vec![0; q];
    while let Some((c, i)) = nt.pop_front() {
        res[i] = c;
    }

    for i in 0..q {
        let root = uf.root(i);
        println!("{}", res[root] + g);
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
