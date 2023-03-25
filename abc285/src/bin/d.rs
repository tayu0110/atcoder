#![allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars},
    *,
};

fn main() {
    input! {n: usize, p: [(String, String); n]}

    let mut map = std::collections::HashMap::new();
    for (s, t) in &p {
        map.insert(s.clone(), 0);
        map.insert(t.clone(), 0);
    }
    let mut cnt = 0;
    for (_, v) in map.iter_mut() {
        *v = cnt;
        cnt += 1;
    }

    let mut g = vec![vec![]; cnt];
    let mut ins = vec![0; cnt];
    for (s, t) in p {
        let (s, t) = (*map.get(&s).unwrap(), *map.get(&t).unwrap());
        g[s].push(t);
        ins[t] += 1;
        if ins[t] > 1 {
            println!("No");
            return;
        }
    }

    let mut check = vec![false; cnt];
    for i in 0..cnt {
        if ins[i] != 0 {
            continue;
        }

        let mut nt = std::collections::VecDeque::new();
        nt.push_back(i);
        while let Some(now) = nt.pop_front() {
            if check[now] {
                println!("No");
                return;
            }

            check[now] = true;
            for &to in &g[now] {
                nt.push_back(to);
            }
        }
    }

    if check.into_iter().all(|v| v) {
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
