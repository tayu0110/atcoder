use proconio::input;

fn main() {
    input! {n: usize, m: usize, p: [(usize, usize); m], k: usize, x: [usize; k]}

    let mut uf = unionfind::UnionFind::new(n);
    let set = x
        .into_iter()
        .map(|x| x - 1)
        .collect::<std::collections::HashSet<_>>();
    for &(u, v) in p
        .iter()
        .enumerate()
        .filter(|(i, _)| !set.contains(i))
        .map(|(_, v)| v)
    {
        uf.merge(u - 1, v - 1);
    }

    let mut t = vec![vec![]; n];
    for (u, v) in p
        .into_iter()
        .enumerate()
        .filter(|(i, _)| set.contains(i))
        .map(|(_, (u, v))| (u - 1, v - 1))
    {
        if !uf.is_same(u, v) {
            t[uf.root(u)].push(uf.root(v));
            t[uf.root(v)].push(uf.root(u));
        }
    }

    let res = t.iter().filter(|&v| v.len() % 2 == 1).count();
    if res == 0 || res == 2 {
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
