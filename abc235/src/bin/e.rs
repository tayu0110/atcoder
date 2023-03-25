#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, m: usize, nq: usize, p: [(usize, usize, usize); m], q: [(usize, usize, usize); nq]}

    let mut p = p.into_iter().map(|(a, b, c)| (c, a, b, nq)).collect::<Vec<_>>();
    p.append(&mut q.into_iter().enumerate().map(|(i, (a, b, c))| (c, a, b, i)).collect());

    p.sort();

    let mut res = vec![false; nq];
    let mut uf = UnionFind::new(n);

    for (_, a, b, i) in p {
        if !uf.is_same(a-1, b-1) {
            if i < nq {
                res[i] = true;
            } else {
                uf.merge(a-1, b-1);
            }
        }
    }

    for v in res {
        if v {
            println!("Yes");
        } else {
            println!("No");
        }
    }
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
            let tmp = rl;
            rl = rr;
            rr = tmp;
        }
        self.tree[rl] += self.tree[rr];
        self.tree[rr] = rl as i32;
        return true;
    }
}
