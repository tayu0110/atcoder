#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

struct UnionFind {
    tree: Vec<i32>
}
impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind { tree: vec![-1; size] }
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
        if rr > rl {
            std::mem::swap(&mut rr, &mut rl);
        }
        self.tree[rl] += self.tree[rr];
        self.tree[rr] = rl as i32;
        true
    }
}

fn main() {
    input! {n: usize, m: usize, e: usize, p: [(usize, usize); e], q: usize, x: [usize; q]};

    let mut uf = UnionFind::new(n+m);
    let mut ck = std::collections::HashSet::new();
    for v in &x {
        ck.insert(*v-1);
    }
    for (i, &(u, v)) in p.iter().enumerate() {
        let (u, v) = (u-1, v-1);
        if ck.contains(&i) {
            continue;
        }
        uf.merge(u, v);
    }
    let mut now = 0;
    for i in 0..n {
        if uf.root(i) >= n {
            now += 1;
        }
    }
    let mut res = vec![];
    for v in x.into_iter().rev() {
        res.push(now);
        let (u, v) = p[v-1];
        let (u, v) = (u-1, v-1);
        if uf.root(u) >= n && uf.root(v) < n {
            now += uf.size(v);
        } else if uf.root(u) < n && uf.root(v) >= n {
            now += uf.size(u);
        }
        uf.merge(u, v);
    }
    while let Some(r) = res.pop() {
        println!("{}", r);
    }
}
