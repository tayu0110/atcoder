#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, m: usize, q: usize, e: [(usize, usize, i64); m], p: [(usize, usize); q]}

    let mut t = vec![vec![]; n];
    let mut rt = vec![vec![]; n];
    let mut uf = unionfind::UnionFind::new(n);
    for (a, b, c) in e {
        t[a - 1].push((b - 1, c));
        rt[b - 1].push((a - 1, -c));
        uf.merge(a - 1, b - 1);
    }

    let mut in_e = vec![0; n];
    for i in 0..n {
        for &(to, _) in &t[i] {
            in_e[to] += 1;
        }
    }

    let mut state = vec![0u8; n];
    let mut checked = vec![false; n];
    let mut dist = std::collections::HashMap::new();
    for i in 0..n {
        if state[uf.root(i)] == 3 {
            continue;
        }

        if checked[uf.root(i)] {
            continue;
        }

        let mut nt = std::collections::VecDeque::new();
        let mut checked_list = std::collections::HashSet::new();
        nt.push_back((i, 0));
        'base: while let Some((now, nd)) = nt.pop_front() {
            if let Some(&d) = dist.get(&now) {
                if d != nd {
                    state[uf.root(i)] = 3;
                    break;
                }
                continue;
            }
            dist.insert(now, nd);
            checked_list.insert(now);

            for &(to, c) in &t[now] {
                if let Some(&td) = dist.get(&to) {
                    if td != nd + c {
                        state[uf.root(i)] = 3;
                        break 'base;
                    } else {
                        continue;
                    }
                }
                nt.push_back((to, nd + c));
            }
            for &(to, c) in &rt[now] {
                if let Some(&td) = dist.get(&to) {
                    if td != nd + c {
                        state[uf.root(i)] = 3;
                        break 'base;
                    } else {
                        continue;
                    }
                }
                nt.push_back((to, nd + c));
            }
        }

        for k in checked_list {
            checked[k] = true;
        }
        if state[uf.root(i)] == 0 {
            state[uf.root(i)] = 2;
        }
    }

    for (x, y) in p {
        if !uf.is_same(x - 1, y - 1) {
            println!("nan");
            continue;
        }

        if state[uf.root(x - 1)] == 3 {
            println!("inf");
            continue;
        }

        let dx = dist.get(&(x - 1)).unwrap();
        let dy = dist.get(&(y - 1)).unwrap();
        println!("{}", dy - dx);
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
