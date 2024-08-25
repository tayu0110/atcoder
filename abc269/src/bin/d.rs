#[allow(unused_imports)]
use proconio::{input, marker::Chars, source::line::LineSource};

fn main() {
    input! {n: usize, p: [(i32, i32); n]}

    let map = p.iter().enumerate().map(|(l, r)| (*r, l)).collect::<std::collections::HashMap<_, _>>();

    let mut uf = UnionFind::new(n);
    for (k, (x, y)) in p.into_iter().enumerate() {
        for (i, j) in [(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)] {
            let (nx, ny) = (x + i, y + j);
            if let Some(idx) = map.get(&(nx, ny)) {
                uf.merge(k, *idx);
            }
        }
    }

    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        let root = uf.root(i);
        set.insert(root);
    }

    println!("{}", set.len());
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