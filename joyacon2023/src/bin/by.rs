use proconio::input;

fn main() {
    input! {n: usize, p: [(i64, i64); n]}

    let map = p
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<std::collections::HashMap<_, _>>();
    let mut uf = unionfind::UnionFind::new(n);

    for (i, (x, y)) in p.into_iter().enumerate() {
        for (dx, dy) in vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)] {
            let (nx, ny) = (x + dx, y + dy);
            if let Some(idx) = map.get(&(nx, ny)) {
                uf.merge(i, *idx);
            }
        }
    }

    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        set.insert(uf.root(i));
    }

    println!("{}", set.len())
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
