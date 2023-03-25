use proconio::*;

fn main() {
    input! {n: usize, p: [(usize, usize); n]}

    let mut xs = vec![];
    let mut ys = vec![];
    for (i, (x, y)) in p.into_iter().enumerate() {
        xs.push((x, i));
        ys.push((y, i));
    }

    xs.sort();
    ys.sort();

    let mut edges = vec![];
    for v in xs.windows(2) {
        let (x0, l) = v[0];
        let (x1, r) = v[1];
        edges.push((x1 - x0, l, r));
    }
    for v in ys.windows(2) {
        let (y0, l) = v[0];
        let (y1, r) = v[1];
        edges.push((y1 - y0, l, r));
    }

    edges.sort();
    let mut res = 0;
    let mut uf = unionfind::UnionFind::new(n);
    for (c, l, r) in edges {
        if !uf.is_same(l, r) {
            uf.merge(l, r);
            res += c;
        }
    }

    println!("{}", res)
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
