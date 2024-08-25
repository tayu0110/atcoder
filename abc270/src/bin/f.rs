#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn solve(n: usize, edges: &Vec<(usize, usize, usize)>) -> usize {
    let mut uf = UnionFind::new(n);

    let mut res = 0;
    for (cost, from, to) in edges {
        if !uf.is_same(*from, *to) {
            uf.merge(*from, *to);
            res += *cost;
        }
    }

    let root = uf.root(0);
    for i in 1..n {
        if uf.root(i) != root {
            res = std::usize::MAX;
        }
    }

    res
}

fn main() {
    input! {n: usize, m: usize, x: [usize; n], y: [usize; n], p: [(usize, usize, usize); m]}

    let mut edges = p.iter().map(|(u, v, c)| (*c, *u-1, *v-1)).collect::<Vec<_>>();
    edges.sort();
    let mut res = solve(n, &edges);

    // eprintln!("res: {}", res);

    let mut e = edges.clone();
    for (i, v) in x.iter().enumerate() {
        e.push((*v, i, n));
    }
    e.sort();
    res = std::cmp::min(res, solve(n+1, &e));
    // eprintln!("res: {}", res);

    let mut e = edges.clone();
    for (i, v) in y.iter().enumerate() {
        e.push((*v, i, n));
    }
    e.sort();
    res = std::cmp::min(res, solve(n+1, &e));
    // eprintln!("res: {}", res);

    for (i, v) in x.into_iter().enumerate() {
        e.push((v, i, n+1));
    }
    e.sort();
    res = std::cmp::min(res, solve(n+2, &e));
    // eprintln!("res: {}", res);

    println!("{}", res);
}

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
