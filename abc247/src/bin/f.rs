#![allow(unused_imports)]
use proconio::{*, input, marker::{Chars, Bytes}, source::line::LineSource};
use itertools::Itertools;

const MOD: usize = 998244353;
fn solve(n: usize) -> usize {
    let mut dp = vec![vec![vec![0usize; 2]; 2]; n];

    dp[0][0][0] = 1;
    dp[0][1][1] = 1;

    for i in 0..n-1 {
        for covered in 0..2 {
            for top in 0..2 {
                if covered == 0 {
                    dp[i+1][1][top] += dp[i][covered][top];
                    dp[i+1][1][top] %= MOD;
                } else {
                    dp[i+1][0][top] += dp[i][covered][top];
                    dp[i+1][0][top] %= MOD;
                    dp[i+1][1][top] += dp[i][covered][top];
                    dp[i+1][1][top] %= MOD;
                }
            }
        }
    }

    (dp[n-1][0][1] + dp[n-1][1][1] + dp[n-1][1][0]) % MOD
}

#[fastout]
fn main() {
    input! {n: usize, p: [usize; n], q: [usize; n]}

    let mut uf = unionfind::UnionFind::new(n);
    for (np, nq) in p.into_iter().zip(q.into_iter()) {
        uf.merge(np-1, nq-1);
    }

    let mut res = 1;
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        let root = uf.root(i);
        if !set.contains(&root) {
            set.insert(root);

            let size = uf.size(root);
            if size == 1 {
                continue;
            }

            res *= solve(size);
            res %= MOD;
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
