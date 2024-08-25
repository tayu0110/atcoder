use std::collections::HashSet;

use proconio::input;

struct UnionFind {
    tree: Vec<i32>
}
impl UnionFind {
    fn new(size: usize) -> Self {
        let mut tree = vec![];
        for _ in 0..size {
            tree.push(-1);
        }
        UnionFind { tree }
    }
    #[allow(dead_code)]
    fn root(&self, index: usize) -> usize {
        if self.tree[index] < 0 {
            index
        } else {
            self.root(self.tree[index] as usize)
        }
    }
    #[allow(dead_code)]
    fn size(&self, index: usize) -> usize {
        -self.tree[self.root(index)] as usize
    }
    #[allow(dead_code)]
    fn is_same(&self, left: usize, right: usize) -> bool {
        self.root(left) == self.root(right)
    }
    #[allow(dead_code)]
    fn merge(&mut self, left: usize, right: usize) -> bool {
        let mut rl = self.root(left);
        let mut rr = self.root(right);
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

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize, usize); m]};

    let mut p = p;
    p.sort();

    let mut uf = UnionFind::new(n+1);
    let mut res = 0;
    for (c, l, r) in p {
        if !uf.is_same(l-1, r) {
            res += c;
            uf.merge(l-1, r);
        }
    }

    let mut ck = HashSet::new();
    for i in 0..=n {
        ck.insert(uf.root(i));
    }

    if ck.len() > 1 {
        println!("-1");
    } else {
        println!("{}", res);
    }
}