#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, q: usize, c: [usize; n], p: [(usize, usize, usize); q]}

    let mut uf = UnionFind::new(n, c);
    for (t, x, y) in p {
        if t == 1 {
            uf.merge(x-1, y-1);
        } else {
            println!("{}", uf.class(x-1, y));
        }
    }
}

struct UnionFind {
    tree: Vec<i32>,
    group: Vec<std::collections::HashMap<usize, usize>>
}
impl UnionFind {
    fn new(size: usize, c: Vec<usize>) -> Self {
        let tree = vec![-1; size];
        let group = c.into_iter().map(|v| {
            let mut map = std::collections::HashMap::new();
            map.insert(v, 1usize);
            map
        }).collect();
        UnionFind { tree, group }
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

        for (k, v) in self.group[rr].clone() {
            *self.group[rl].entry(k).or_insert(0) += v;
        }
        true
    }
    fn class(&self, idx: usize, c: usize) -> usize {
        let root = self.root(idx);
        if let Some(res) = self.group[root].get(&c) {
            *res
        } else {
            0
        }
    }
}